use crate::monitor::DetailedMetrics;

pub trait Notifier {
    fn notify(&self, message: &str);
}

pub struct AlertRule {
    pub name: String,
    pub condition: Box<dyn Fn(&DetailedMetrics) -> bool>,
    pub message: Box<dyn Fn(&DetailedMetrics) -> String>,
}

pub struct AlertManager {
    rules: Vec<AlertRule>,
    notifiers: Vec<Box<dyn Notifier>>,
}

impl AlertManager {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            notifiers: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule: AlertRule) {
        self.rules.push(rule);
    }

    pub fn add_notifier(&mut self, notifier: Box<dyn Notifier>) {
        self.notifiers.push(notifier);
    }

    pub fn check_alerts(&self, metrics: &DetailedMetrics) {
        for rule in &self.rules {
            if (rule.condition)(metrics) {
                let message = (rule.message)(metrics);
                for notifier in &self.notifiers {
                    notifier.notify(&message);
                }
            }
        }
    }
} 