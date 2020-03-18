use std::borrow::Borrow;
use std::collections::HashMap;

use crate::function::{BinaryFunction, Function, UnaryFunction};
use crate::num::checked::CheckedNum;
use crate::ops::checked::*;
use crate::ops::math::*;
use crate::utils::ignore_case_string::IgnoreCaseString;

pub trait Context<'a, N> {
    fn config(&self) -> &Config;

    fn add_function<F: Function<N> + 'a>(&mut self, func: F);

    fn add_binary_function<F: BinaryFunction<N> + 'a>(&mut self, func: F);

    fn add_unary_function<F: UnaryFunction<N> + 'a>(&mut self, func: F);

    fn add_constant(&mut self, name: &str, value: N);

    fn get_variable(&self, name: &str) -> Option<&N>;

    fn set_variable(&mut self, name: &str, value: N) -> Option<N>;

    fn get_constant(&self, name: &str) -> Option<&N>;

    fn get_function(&self, name: &str) -> Option<&Box<dyn Function<N> + 'a>>;

    fn get_binary_function(&self, name: &str) -> Option<&Box<dyn BinaryFunction<N> + 'a>>;

    fn get_unary_function(&self, name: &str) -> Option<&Box<dyn UnaryFunction<N> + 'a>>;

    /// Checks if exists a variable with the given name.
    #[inline]
    fn is_variable(&self, name: &str) -> bool {
        match self.get_variable(name) {
            Some(_) => true,
            None => false,
        }
    }

    /// Checks if exists a constant with the given name.
    #[inline]
    fn is_constant(&self, name: &str) -> bool {
        match self.get_constant(name) {
            Some(_) => true,
            None => false,
        }
    }

    /// Checks if exists a function with the given name.
    #[inline]
    fn is_function(&self, name: &str) -> bool {
        match self.get_function(name) {
            Some(_) => true,
            None => false,
        }
    }

    /// Checks if exists a binary function with the given name.
    #[inline]
    fn is_binary_function(&self, name: &str) -> bool {
        match self.get_binary_function(name) {
            Some(_) => true,
            None => false,
        }
    }

    /// Checks if exists a unary function with the given name.
    #[inline]
    fn is_unary_function(&self, name: &str) -> bool {
        match self.get_unary_function(name) {
            Some(_) => true,
            None => false,
        }
    }
}

/// Provides a default implementation of the `MathContext`,
/// which can be create providing all the variables, constants and functions of the context.
pub struct DefaultContext<'a, N> {
    /// The variables.
    variables: HashMap<IgnoreCaseString, N>,
    /// The constants.
    constants: HashMap<IgnoreCaseString, N>,
    /// The functions.
    functions: HashMap<IgnoreCaseString, Box<dyn Function<N> + 'a>>,
    /// The binary functions.
    binary_functions: HashMap<IgnoreCaseString, Box<dyn BinaryFunction<N> + 'a>>,
    /// The unary functions.
    unary_functions: HashMap<IgnoreCaseString, Box<dyn UnaryFunction<N> + 'a>>,
    /// Additional information about this context
    config: Config,
}

impl<'a, N> DefaultContext<'a, N> {
    #[inline]
    pub fn new() -> Self{
        Self::new_with_config(Config::new())
    }

    #[inline]
    pub fn new_with_config(config: Config) -> Self {
        DefaultContext {
            variables: Default::default(),
            constants: Default::default(),
            functions: Default::default(),
            binary_functions: Default::default(),
            unary_functions: Default::default(),
            config,
        }
    }

    /// Creates a new empty `Context`.
    #[inline]
    pub fn empty() -> Self {
        DefaultContext {
            variables: Default::default(),
            constants: Default::default(),
            functions: Default::default(),
            binary_functions: Default::default(),
            unary_functions: Default::default(),
            config: Config::default(),
        }
    }

    /// Creates a new empty `Context` using the given `Config`.
    #[inline]
    pub fn empty_with_config(config: Config) -> Self {
        DefaultContext {
            variables: Default::default(),
            constants: Default::default(),
            functions: Default::default(),
            binary_functions: Default::default(),
            unary_functions: Default::default(),
            config,
        }
    }

    #[inline]
    pub fn variables(&self) -> &HashMap<IgnoreCaseString, N> {
        &self.variables
    }

    #[inline]
    pub fn constants(&self) -> &HashMap<IgnoreCaseString, N> {
        &self.constants
    }

    #[inline]
    pub fn functions(&self) -> &HashMap<IgnoreCaseString, Box<dyn Function<N> + 'a>> {
        &self.functions
    }

    #[inline]
    pub fn binary_functions(&self) -> &HashMap<IgnoreCaseString, Box<dyn BinaryFunction<N> + 'a>> {
        &self.binary_functions
    }

    #[inline]
    pub fn unary_functions(&self) -> &HashMap<IgnoreCaseString, Box<dyn UnaryFunction<N> + 'a>> {
        &self.unary_functions
    }

    fn add_function_as<F: Function<N> + 'a>(&mut self, name: &str, func: F) {
        let function_name = IgnoreCaseString::from(name);
        match self.functions.contains_key(&function_name){
            true => panic!("A function named '{}' already exists", function_name),
            false => self.functions.insert(function_name, Box::new(func))
        };
    }

    fn add_binary_function_as<F: BinaryFunction<N> + 'a>(&mut self, name: &str, func: F) {
        let function_name = IgnoreCaseString::from(name);
        match self.binary_functions.contains_key(&function_name){
            true => panic!("A binary function named '{}' already exists", function_name),
            false => self.binary_functions.insert(function_name, Box::new(func))
        };
    }

    fn add_unary_function_as<F: UnaryFunction<N> + 'a>(&mut self, name: &str, func: F) {
        let function_name = IgnoreCaseString::from(name);
        match self.unary_functions.contains_key(&function_name){
            true => panic!("An unary function named '{}' already exists", function_name),
            false => self.unary_functions.insert(function_name, Box::new(func))
        };
    }
}

impl<'a, N> Context<'a, N> for DefaultContext<'a, N> {
    fn config(&self) -> &Config {
        &self.config
    }

    #[inline]
    fn add_function<F: Function<N> + 'a>(&mut self, func: F) {
        self.add_function_as(&func.name().to_string(), func)
    }

    #[inline]
    fn add_binary_function<F: BinaryFunction<N> + 'a>(&mut self, func: F) {
        self.add_binary_function_as(&func.name().to_string(), func)
    }

    #[inline]
    fn add_unary_function<F: UnaryFunction<N> + 'a>(&mut self, func: F) {
        self.add_unary_function_as(&func.name().to_string(), func)
    }

    #[inline]
    fn add_constant(&mut self, name: &str, value: N) {
        self.constants.insert(IgnoreCaseString::from(name), value);
    }

    #[inline]
    fn get_variable(&self, name: &str) -> Option<&N> {
        self.variables.get(IgnoreCaseString::from(name).borrow())
    }

    #[inline]
    fn set_variable(&mut self, name: &str, value: N) -> Option<N> {
        let string = IgnoreCaseString::from(name);
        match self.constants.contains_key(&string){
            true => panic!("Invalid variable name, a constant named '{}' already exists", string.clone()),
            false => self.variables.insert(string, value)
        }
    }

    #[inline]
    fn get_constant(&self, name: &str) -> Option<&N> {
        self.constants.get(&IgnoreCaseString::from(name))
    }

    #[inline]
    fn get_function(&self, name: &str) -> Option<&Box<dyn Function<N> + 'a>> {
        self.functions.get(&IgnoreCaseString::from(name))
    }

    #[inline]
    fn get_binary_function(&self, name: &str) -> Option<&Box<dyn BinaryFunction<N> + 'a>> {
        self.binary_functions.get(&IgnoreCaseString::from(name))
    }

    #[inline]
    fn get_unary_function(&self, name: &str) -> Option<&Box<dyn UnaryFunction<N> + 'a>> {
        self.unary_functions.get(&IgnoreCaseString::from(name))
    }
}

impl<'a, N: CheckedNum> DefaultContext<'a, N> {
    pub fn instance() -> &'static DefaultContext<'a, N>{
        use crate::utils::lazy::Lazy;
        use crate::utils::untyped::Untyped;;
        use std::any::TypeId;;

        static mut CACHE : Lazy<HashMap<TypeId, Untyped>> = Lazy::new(|| HashMap::new());

        let id = TypeId::of::<N>();
        unsafe{
            match (*CACHE).get(&id){
                Some(p) => {
                    p.cast::<DefaultContext<'a, N>>()
                },
                None => {
                    let context = Box::leak(Box::new(DefaultContext::new_checked()));
                    CACHE.insert(id, Untyped::new(context));
                    context
                }
            }
        }
    }

    /// Creates a new `Context` with the default functions and constants.
    #[inline]
    pub fn new_checked() -> Self {
        Self::new_checked_with_config(Config::new())
    }

    /// Creates a new `Context` with the default functions and constants using the specified `Config`.
    pub fn new_checked_with_config(config: Config) -> Self {
        let mut context = Self::new_with_config(config);
        context.add_constant("PI", N::from_f64(std::f64::consts::PI).unwrap());
        context.add_constant("E", N::from_f64(std::f64::consts::E).unwrap());
        context.add_binary_function(AddOperator);
        context.add_binary_function(SubOperator);
        context.add_binary_function(MulOperator);
        context.add_binary_function(DivOperator);
        context.add_binary_function(PowOperator);
        context.add_binary_function(ModOperator);
        context.add_unary_function(UnaryPlus);
        context.add_unary_function(UnaryMinus);
        context.add_unary_function(Factorial);
        context.add_function(SumFunction);
        context.add_function(ProdFunction);
        context.add_function(AvgFunction);
        context.add_function(MaxFunction);
        context.add_function(MinFunction);
        context.add_function(AbsFunction);
        context.add_function(SqrtFunction);
        context.add_function(LnFunction);
        context.add_function(LogFunction);
        context.add_function(ExpFunction);
        context.add_function(FloorFunction);
        context.add_function(CeilFunction);
        context.add_function(TruncateFunction);
        context.add_function(RoundFunction);
        context.add_function(SignFunction);
        context.add_function(SinFunction);
        context.add_function(CosFunction);
        context.add_function(TanFunction);
        context.add_function(CscFunction);
        context.add_function(SecFunction);
        context.add_function(CotFunction);
        context.add_function(ASinFunction);
        context.add_function(ACosFunction);
        context.add_function(ATanFunction);
        context.add_function(ACscFunction);
        context.add_function(ASecFunction);
        context.add_function(ACotFunction);
        context.add_function(SinhFunction);
        context.add_function(CoshFunction);
        context.add_function(TanhFunction);
        context.add_function(CschFunction);
        context.add_function(SechFunction);
        context.add_function(CothFunction);
        context.add_function(ASinhFunction);
        context.add_function(ACoshFunction);
        context.add_function(ATanhFunction);
        context.add_function(ACschFunction);
        context.add_function(ASechFunction);
        context.add_function(ACothFunction);
        #[cfg(feature = "random")]
        {
            use crate::random::RandFunction;
            context.add_function(RandFunction);
        }
        context
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct GroupingSymbol {
    pub group_open: char,
    pub group_close: char,
}

impl GroupingSymbol {
    #[inline]
    pub fn new(group_open: char, group_close: char) -> Self {
        assert_ne!(group_open, group_close);
        GroupingSymbol {
            group_open,
            group_close,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Config {
    implicit_mul: bool,
    complex_number: bool,
    grouping: HashMap<char, GroupingSymbol>,
}

impl Config {
    #[inline]
    pub fn new() -> Self {
        Config::default()
            .with_group_symbol('(', ')')
    }

    #[inline]
    pub fn with_implicit_mul(mut self) -> Config {
        self.implicit_mul = true;
        self
    }

    #[inline]
    pub fn with_complex_number(mut self) -> Config {
        self.complex_number = true;
        self
    }

    pub fn with_group_symbol(mut self, open_group: char, close_group: char) -> Config {
        let grouping = &mut self.grouping;
        let grouping_symbol = GroupingSymbol::new(open_group, close_group);
        grouping
            .insert(open_group, grouping_symbol)
            .map(|_| panic!("Duplicated symbol: `{}`", open_group));
        grouping
            .insert(close_group, grouping_symbol)
            .map(|_| panic!("Duplicated symbol: `{}`", close_group));
        self
    }

    pub fn implicit_mul(&self) -> bool {
        self.implicit_mul
    }

    pub fn complex_number(&self) -> bool {
        self.complex_number
    }

    pub fn get_group_symbol(&self, symbol: char) -> Option<&GroupingSymbol> {
        self.grouping.get(&symbol)
    }
}

impl Default for Config{
    fn default() -> Self {
        Config{
            implicit_mul: false,
            complex_number: false,
            grouping: Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_context_test() {
        let context: DefaultContext<f64> = DefaultContext::new_checked();

        let a = context.get_constant("E").unwrap();
        let b = context.get_constant("e").unwrap();
        assert_eq!(a, b);

        assert!(context.get_constant("Pi").is_some());
        assert!(context.get_binary_function("+").is_some());
        assert!(context.get_binary_function("-").is_some());
        assert!(context.get_binary_function("/").is_some());
        assert!(context.get_binary_function("*").is_some());

        assert!(context.get_function("SUM").is_some());
        assert!(context.get_function("AvG").is_some());
        assert!(context.get_function("Max").is_some());
        assert!(context.get_function("min").is_some());
    }

    #[test]
    fn config_test() {
        let config = Config::default()
            .with_group_symbol('(', ')')
            .with_group_symbol('[', ']');

        assert_eq!(
            config.get_group_symbol('(').unwrap(),
            &GroupingSymbol::new('(', ')')
        );
        assert_eq!(
            config.get_group_symbol(')').unwrap(),
            &GroupingSymbol::new('(', ')')
        );
        assert_eq!(
            config.get_group_symbol('[').unwrap(),
            &GroupingSymbol::new('[', ']')
        );
        assert_eq!(
            config.get_group_symbol(']').unwrap(),
            &GroupingSymbol::new('[', ']')
        );
    }
}

pub mod unchecked {
    use crate::context::{Config, Context, DefaultContext};
    use crate::num::unchecked::UncheckedNum;
    use crate::ops::math::*;
    use crate::ops::unchecked::*;

    impl <'a, N> DefaultContext<'a, N> where N : UncheckedNum{
        /// Creates a new `Context` with the default functions and constants.
        #[inline]
        pub fn new_unchecked() -> Self {
            Self::new_unchecked_with_config(Config::new())
        }

        /// Creates a new `Context` with the default functions and constants using the specified `Config`.
        pub fn new_unchecked_with_config(config: Config) -> Self {
            let mut context = Self::new_with_config(config);
            context.add_constant("PI", N::from_f64(std::f64::consts::PI).unwrap());
            context.add_constant("E", N::from_f64(std::f64::consts::E).unwrap());
            context.add_binary_function(AddOperator);
            context.add_binary_function(SubOperator);
            context.add_binary_function(MulOperator);
            context.add_binary_function(DivOperator);
            context.add_binary_function(PowOperator);
            context.add_binary_function(ModOperator);
            context.add_unary_function(UnaryPlus);
            context.add_unary_function(UnaryMinus);
            context.add_unary_function(Factorial);
            context.add_function(SumFunction);
            context.add_function(AvgFunction);
            context.add_function(ProdFunction);
            context.add_function(MaxFunction);
            context.add_function(MinFunction);
            context.add_function(SqrtFunction);
            context.add_function(LnFunction);
            context.add_function(LogFunction);
            context.add_function(ExpFunction);
            context.add_function(SinFunction);
            context.add_function(CosFunction);
            context.add_function(TanFunction);
            context.add_function(CscFunction);
            context.add_function(SecFunction);
            context.add_function(CotFunction);
            context.add_function(ASinFunction);
            context.add_function(ACosFunction);
            context.add_function(ATanFunction);
            context.add_function(ACscFunction);
            context.add_function(ASecFunction);
            context.add_function(ACotFunction);
            context.add_function(SinhFunction);
            context.add_function(CoshFunction);
            context.add_function(TanhFunction);
            context.add_function(CschFunction);
            context.add_function(SechFunction);
            context.add_function(CothFunction);
            context.add_function(ASinhFunction);
            context.add_function(ACoshFunction);
            context.add_function(ATanhFunction);
            context.add_function(ACschFunction);
            context.add_function(ASechFunction);
            context.add_function(ACothFunction);
            context
        }
    }
}
