use std::rc::Rc;

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a> {
    pub logger: &'a dyn Logger,
    pub value: Rc<usize>,
    pub max: usize,
}

impl<'a> Tracker<'a> {
    pub fn new(logger: &'a dyn Logger, max: usize) -> Tracker<'a> {
        Tracker {
            logger,
            value: Rc::new(0),
            max,
        }
    }

    pub fn set_value(&'a self, value: &Rc<usize>) {
        let percent = (Rc::strong_count(&value) * 100) / self.max;

        if percent >= 100 {
            self.logger
                .error(format!("Error: you are over your quota!").as_str());
        } else if percent >= 70 {
            self.logger.warning(
                format!(
                    "Warning: you have used up over {}% of your quota! Proceeds with precaution",
                    percent
                )
                .as_str(),
            );
        }
    }

    pub fn peek(&'a self, value: &Rc<usize>) {
        let percent = (Rc::strong_count(&value) * 100) / self.max;

        self.logger
            .info(format!("Info: you are using up to {}% of your quota", percent).as_str());
    }
}
