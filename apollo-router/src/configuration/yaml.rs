use crate::configuration::ConfigurationError;
use derivative::Derivative;
use indexmap::IndexMap;
use jsonschema::paths::{JSONPointer, PathChunk};
use std::collections::HashMap;
use yaml_rust::parser::{MarkedEventReceiver, Parser};
use yaml_rust::scanner::Marker;
use yaml_rust::Event;

#[derive(Derivative, Clone, Debug, Eq)]
#[derivative(Hash, PartialEq)]
pub(crate) struct Label {
    pub(crate) name: String,
    #[derivative(Hash = "ignore", PartialEq = "ignore")]
    pub(crate) marker: Option<Marker>,
}

impl From<String> for Label {
    fn from(name: String) -> Self {
        Label { name, marker: None }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Value {
    // These types are not currently used.
    // In theory if we want to parse the yaml properly then we need them, but we're only interrested
    // in the markers, so maybe we don't need them?
    // Null(Marker),
    // Bool(bool, Marker),
    // Number(Number, Marker),
    String(String, Marker),
    Sequence(Vec<Value>, Marker),
    Mapping(Option<Label>, IndexMap<Label, Value>, Marker),
}

impl Value {
    pub(crate) fn end_marker(&self) -> &Marker {
        match self {
            Value::String(_, m) => m,
            Value::Sequence(v, m) => v.last().map(|l| l.end_marker()).unwrap_or_else(|| m),
            Value::Mapping(_, v, m) => v
                .last()
                .map(|(_, val)| val.end_marker())
                .unwrap_or_else(|| m),
        }
    }
}

/// A basic yaml parser that retains marker information.
/// This is an incomplete parser that is useful for config validation.
/// First the yaml is loaded via serde_yaml. This ensures valid yaml.
/// Then it is validated against a json schema.
/// The output from json schema validation is a set of errors with json paths.
/// The json path doesn't contain line number info, so we reparse so that we can convert the
/// paths into nice error messages.
#[derive(Default, Debug)]
pub(crate) struct MarkedYaml {
    anchors: HashMap<usize, Value>,
    current_label: Option<Label>,
    object_stack: Vec<(Option<Label>, Value, usize)>,
    root: Option<Value>,
}

impl MarkedYaml {
    pub(crate) fn get_element(&self, pointer: &JSONPointer) -> Option<&Value> {
        let mut current = self.root();
        for item in pointer.iter() {
            current = match (current, item) {
                (Some(Value::Mapping(_current_label, mapping, _)), PathChunk::Property(value)) => {
                    mapping.get(&Label::from(value.to_string()))
                }
                (Some(Value::Sequence(sequence, _)), PathChunk::Index(idx)) => sequence.get(*idx),
                _ => None,
            }
        }
        current
    }

    fn root(&self) -> Option<&Value> {
        self.root.as_ref()
    }

    fn end_container(&mut self) -> Option<Value> {
        let (label, v, id) = self.object_stack.pop().expect("imbalanced parse events");
        self.anchors.insert(id, v.clone());
        match (label, self.object_stack.last_mut()) {
            (Some(label), Some((_, Value::Mapping(_current_label, mapping, _), _))) => {
                mapping.insert(label, v);
                None
            }
            (None, Some((_, Value::Sequence(sequence, _), _))) => {
                sequence.push(v);
                None
            }
            _ => Some(v),
        }
    }

    fn add_value(&mut self, marker: Marker, v: String, id: usize) {
        match (self.current_label.take(), self.object_stack.last_mut()) {
            (Some(label), Some((_, Value::Mapping(_current_label, mapping, _), _))) => {
                let v = Value::String(v, marker);
                self.anchors.insert(id, v.clone());
                mapping.insert(label, v);
            }
            (None, Some((_, Value::Sequence(sequence, _), _))) => {
                let v = Value::String(v, marker);
                self.anchors.insert(id, v.clone());
                sequence.push(v);
            }
            (None, _) => {
                self.current_label = Some(Label {
                    name: v,
                    marker: Some(marker),
                })
            }
            _ => tracing::warn!("labeled scalar without container in yaml"),
        }
    }

    fn add_alias_value(&mut self, v: Value) {
        match (self.current_label.take(), self.object_stack.last_mut()) {
            (Some(label), Some((_, Value::Mapping(_current_label, mapping, _), _))) => {
                mapping.insert(label, v);
            }
            (None, Some((_, Value::Sequence(sequence, _), _))) => {
                sequence.push(v);
            }
            _ => tracing::warn!("scalar without container in yaml"),
        }
    }
}

pub(crate) fn parse(source: &str) -> Result<MarkedYaml, ConfigurationError> {
    // Yaml parser doesn't support CRLF. Remove CRs.
    // https://github.com/chyh1990/yaml-rust/issues/165
    let source = source.replace('\r', "");
    let mut parser = Parser::new(source.chars());
    let mut loader = MarkedYaml::default();
    parser
        .load(&mut loader, true)
        .map_err(|e| ConfigurationError::InvalidConfiguration {
            message: "could not parse yaml",
            error: e.to_string(),
        })?;

    Ok(loader)
}

impl MarkedEventReceiver for MarkedYaml {
    fn on_event(&mut self, ev: Event, marker: Marker) {
        match ev {
            Event::Scalar(v, _style, id, _tag) => self.add_value(marker, v, id),
            Event::SequenceStart(id) => {
                self.object_stack.push((
                    self.current_label.take(),
                    Value::Sequence(Vec::new(), marker),
                    id,
                ));
            }
            Event::SequenceEnd => {
                self.root = self.end_container();
            }
            Event::MappingStart(id) => {
                let current_label = self.current_label.take();
                self.object_stack.push((
                    current_label.clone(),
                    Value::Mapping(current_label, IndexMap::new(), marker),
                    id,
                ));
            }
            Event::MappingEnd => {
                self.root = self.end_container();
            }
            Event::Alias(id) => {
                if let Some(v) = self.anchors.get(&id) {
                    let cloned = v.clone();
                    self.add_alias_value(cloned);
                } else {
                    tracing::warn!("unresolved anchor in yaml");
                }
            }
            Event::DocumentStart => {}
            Event::DocumentEnd => {}
            _ => {}
        }
    }
}

#[cfg(test)]
mod test {
    use crate::configuration::yaml::parse;
    use insta::assert_snapshot;

    #[test]
    fn test() {
        // DON'T reformat this. It'll change the test results
        let yaml = r#"test:
  a: 4
  b: 3       
  c: &id001
  - d
  - e
  - f:
     - g
     - h:
         i: k 
  l: *id001
      
"#;
        let parsed = parse(yaml).unwrap();
        let root = parsed.root().unwrap();
        assert_snapshot!(format!("{:#?}", root));
    }
}
