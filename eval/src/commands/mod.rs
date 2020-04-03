mod eval;
mod run;
mod context;

pub use self::eval::EvalCommand;
pub use self::run::RunCommand;
pub use self::context::ContextCommand;

mod info {
    use std::convert::TryFrom;
    use std::result;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum CommandInfo {
        Eval,
        Run,
        Help,
        Context,
        Ops,
        Func,
        Consts
    }

    impl CommandInfo {
        pub fn name(&self) -> &str{
            match self{
                CommandInfo::Eval => "",
                CommandInfo::Run => "--run",
                CommandInfo::Help => "--help",
                CommandInfo::Context => "--context",
                CommandInfo::Ops => "--operations",
                CommandInfo::Func => "--functions",
                CommandInfo::Consts => "--constants",
            }
        }

        pub fn alias(&self) -> Option<&str>{
            match self{
                CommandInfo::Eval => None,
                CommandInfo::Run => Some("--r"),
                CommandInfo::Help => Some("--h"),
                CommandInfo::Context => Some("--ctx"),
                CommandInfo::Ops => Some("--ops"),
                CommandInfo::Func => Some("--funs"),
                CommandInfo::Consts => Some("--cons"),
            }
        }
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum NumberType {
        Decimal,
        BigDecimal,
        Complex,
    }

    impl Default for NumberType {
        fn default() -> Self {
            NumberType::Decimal
        }
    }

    impl TryFrom<&str> for NumberType {
        type Error = ();

        fn try_from(value: &str) -> result::Result<Self, Self::Error> {
           match value{
                "--decimal" | "--d" => Ok(NumberType::Decimal),
                "--bigdecimal" | "--b" => Ok(NumberType::BigDecimal),
                "--complex" | "--c" => Ok(NumberType::Complex),
                _ => Err(())
            }
        }
    }
}