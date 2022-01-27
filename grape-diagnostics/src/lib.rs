use grape_span::Span;

#[derive(Debug)]
pub struct Diagnostics<'diagnostics> {
    source: &'diagnostics str,
    messages: Vec<Message>,
}

#[derive(Debug, PartialEq)]
pub struct Message {
    kind: MessageKind,
    title: String,
    source: Option<Span>,
}

#[derive(Debug, PartialEq)]
pub struct MessageBuilder {
    kind: MessageKind,
    title: String,
    source: Option<Span>,
}

#[derive(Debug, PartialEq)]
pub enum MessageKind {
    Error,
    Warning,
    Suggestion,
    Help,
    Info,
}

impl<'diagnostics> Diagnostics<'diagnostics> {
    pub const fn new(source: &'diagnostics str) -> Self {
        Diagnostics {
            source,
            messages: vec![],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn span(&self, span: &Span) -> &'diagnostics str {
        &self.source[span.start..span.end]
    }

    pub fn report(&mut self, message: Message) {
        self.messages.push(message);
    }
}

macro_rules! message_builder {
    ($kind:ident, $title:ident) => {
        MessageBuilder {
            kind: MessageKind::$kind,
            title: $title.as_ref().to_owned(),
            source: None,
        }
    };
}

impl MessageBuilder {
    pub fn error<S: AsRef<str>>(title: S) -> Self {
        message_builder!(Error, title)
    }

    pub fn warning<S: AsRef<str>>(title: S) -> Self {
        message_builder!(Warning, title)
    }

    pub fn suggestion<S: AsRef<str>>(title: S) -> Self {
        message_builder!(Suggestion, title)
    }

    pub fn help<S: AsRef<str>>(title: S) -> Self {
        message_builder!(Help, title)
    }

    pub fn info<S: AsRef<str>>(title: S) -> Self {
        message_builder!(Info, title)
    }

    pub fn source(mut self, source: Span) -> Self {
        self.source = Some(source);
        self
    }

    pub fn build(self) -> Message {
        Message {
            kind: self.kind,
            title: self.title,
            source: self.source,
        }
    }
}

impl<'diagnostics> std::fmt::Display for Diagnostics<'diagnostics> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut is_first = true;

        for message in self.messages.iter() {
            if !is_first {
                writeln!(f, "")?;
            } else {
                is_first = false;
            }
            write!(f, "{}: {}", message.kind, message.title)?;
            if let Some(span) = message.source {
                writeln!(f, "")?;
                write!(f, "{}", &self.source[span.start..span.end])?;
            }
        }

        write!(f, "")
    }
}

impl std::fmt::Display for MessageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageKind::Error => write!(f, "error"),
            MessageKind::Warning => write!(f, "warning"),
            MessageKind::Suggestion => write!(f, "suggestion"),
            MessageKind::Info => write!(f, "info"),
            MessageKind::Help => write!(f, "help"),
        }
    }
}
