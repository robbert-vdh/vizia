use super::internal;
use crate::{prelude::*, style::LabelledBy};

pub trait AccessibilityModifiers: internal::Modifiable {
    fn role(mut self, role: Role) -> Self {
        let id = self.entity();
        self.context().style.roles.insert(id, role).unwrap();

        self
    }

    fn name<U: ToString>(mut self, name: impl Res<U>) -> Self {
        let entity = self.entity();
        name.set_or_bind(self.context(), entity, |cx, id, name| {
            cx.style.name.insert(id, name.to_string());
        });

        self
    }

    fn default_action_verb(mut self, action_verb: DefaultActionVerb) -> Self {
        let id = self.entity();
        self.context().style.default_action_verb.insert(id, action_verb).unwrap();

        self
    }

    fn live(mut self, live: Live) -> Self {
        let id = self.entity();

        self.context().style.live.insert(id, live).unwrap();

        self
    }

    fn labelled_by(mut self, labelled_by: LabelledBy) -> Self {
        let id = self.entity();

        self.context().style.labelled_by.insert(id, labelled_by).unwrap();

        self
    }

    fn numeric_value<U: Into<f64>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, id, val| {
            let v = val.into();
            cx.style.numeric_value.insert(id, v).unwrap();
        });

        self
    }

    fn text_value<U: ToString>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, id, val| {
            cx.style.text_value.insert(id, val.to_string()).unwrap();
        });

        self
    }
}

impl<'a, V: View> AccessibilityModifiers for Handle<'a, V> {}
