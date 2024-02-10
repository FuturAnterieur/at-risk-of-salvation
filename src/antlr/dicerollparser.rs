// Generated from DiceRoll.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::dicerolllistener::*;
use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const T__0:isize=1; 
		pub const T__1:isize=2; 
		pub const T__2:isize=3; 
		pub const T__3:isize=4; 
		pub const T__4:isize=5; 
		pub const T__5:isize=6; 
		pub const QUALIFIER:isize=7; 
		pub const NEWLINE:isize=8; 
		pub const NUM:isize=9; 
		pub const DICE_VALUE:isize=10;
	pub const RULE_prog:usize = 0; 
	pub const RULE_diceroll:usize = 1; 
	pub const RULE_qualifiers:usize = 2; 
	pub const RULE_dicerolland:usize = 3; 
	pub const RULE_dicerollor:usize = 4;
	pub const ruleNames: [&'static str; 5] =  [
		"prog", "diceroll", "qualifiers", "dicerolland", "dicerollor"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;8] = [
		None, Some("':'"), Some("'+'"), Some("'{'"), Some("'x'"), Some("'}'"), 
		Some("'or'"), Some("'accum'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;11]  = [
		None, None, None, None, None, None, None, Some("QUALIFIER"), Some("NEWLINE"), 
		Some("NUM"), Some("DICE_VALUE")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,DiceRollParserExt<'input>, I, DiceRollParserContextType , dyn DiceRollListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type DiceRollTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, DiceRollParserContextType , dyn DiceRollListener<'input> + 'a>;

/// Parser for DiceRoll grammar
pub struct DiceRollParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> DiceRollParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				DiceRollParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> DiceRollParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> DiceRollParser<'input, I, DefaultErrorStrategy<'input,DiceRollParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for DiceRollParser
pub trait DiceRollParserContext<'input>:
	for<'x> Listenable<dyn DiceRollListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=DiceRollParserContextType>
{}

antlr_rust::coerce_from!{ 'input : DiceRollParserContext<'input> }

impl<'input> DiceRollParserContext<'input> for TerminalNode<'input,DiceRollParserContextType> {}
impl<'input> DiceRollParserContext<'input> for ErrorNode<'input,DiceRollParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn DiceRollParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn DiceRollListener<'input> + 'input }

pub struct DiceRollParserContextType;
antlr_rust::tid!{DiceRollParserContextType}

impl<'input> ParserNodeType<'input> for DiceRollParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn DiceRollParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for DiceRollParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for DiceRollParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct DiceRollParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> DiceRollParserExt<'input>{
}
antlr_rust::tid! { DiceRollParserExt<'a> }

impl<'input> TokenAware<'input> for DiceRollParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for DiceRollParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for DiceRollParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "DiceRoll.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn DiceRollParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					3 => DiceRollParser::<'input,I,_>::dicerolland_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					4 => DiceRollParser::<'input,I,_>::dicerollor_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> DiceRollParser<'input, I, DefaultErrorStrategy<'input,DiceRollParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn dicerolland_sempred(_localctx: Option<&DicerollandContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 3)
				}
			_ => true
		}
	}
	fn dicerollor_sempred(_localctx: Option<&DicerollorContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				1=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
}
//------------------- prog ----------------
pub type ProgContextAll<'input> = ProgContext<'input>;


pub type ProgContext<'input> = BaseParserRuleContext<'input,ProgContextExt<'input>>;

#[derive(Clone)]
pub struct ProgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DiceRollParserContext<'input> for ProgContext<'input>{}

impl<'input,'a> Listenable<dyn DiceRollListener<'input> + 'a> for ProgContext<'input>{
		fn enter(&self,listener: &mut (dyn DiceRollListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_prog(self);
		}fn exit(&self,listener: &mut (dyn DiceRollListener<'input> + 'a)) {
			listener.exit_prog(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ProgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DiceRollParserContextType;
	fn get_rule_index(&self) -> usize { RULE_prog }
	//fn type_rule_index() -> usize where Self: Sized { RULE_prog }
}
antlr_rust::tid!{ProgContextExt<'a>}

impl<'input> ProgContextExt<'input>{
	fn new(parent: Option<Rc<dyn DiceRollParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProgContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ProgContextAttrs<'input>: DiceRollParserContext<'input> + BorrowMut<ProgContextExt<'input>>{

fn diceroll(&self) -> Option<Rc<DicerollContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,DiceRollParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}

}

impl<'input> ProgContextAttrs<'input> for ProgContext<'input>{}

impl<'input, I, H> DiceRollParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn prog(&mut self,)
	-> Result<Rc<ProgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ProgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_prog);
        let mut _localctx: Rc<ProgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule diceroll*/
			recog.base.set_state(10);
			recog.diceroll()?;

			recog.base.set_state(11);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- diceroll ----------------
pub type DicerollContextAll<'input> = DicerollContext<'input>;


pub type DicerollContext<'input> = BaseParserRuleContext<'input,DicerollContextExt<'input>>;

#[derive(Clone)]
pub struct DicerollContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DiceRollParserContext<'input> for DicerollContext<'input>{}

impl<'input,'a> Listenable<dyn DiceRollListener<'input> + 'a> for DicerollContext<'input>{
		fn enter(&self,listener: &mut (dyn DiceRollListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_diceroll(self);
		}fn exit(&self,listener: &mut (dyn DiceRollListener<'input> + 'a)) {
			listener.exit_diceroll(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for DicerollContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DiceRollParserContextType;
	fn get_rule_index(&self) -> usize { RULE_diceroll }
	//fn type_rule_index() -> usize where Self: Sized { RULE_diceroll }
}
antlr_rust::tid!{DicerollContextExt<'a>}

impl<'input> DicerollContextExt<'input>{
	fn new(parent: Option<Rc<dyn DiceRollParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DicerollContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DicerollContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DicerollContextAttrs<'input>: DiceRollParserContext<'input> + BorrowMut<DicerollContextExt<'input>>{

fn dicerollor(&self) -> Option<Rc<DicerollorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn dicerolland(&self) -> Option<Rc<DicerollandContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn qualifiers(&self) -> Option<Rc<QualifiersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DicerollContextAttrs<'input> for DicerollContext<'input>{}

impl<'input, I, H> DiceRollParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn diceroll(&mut self,)
	-> Result<Rc<DicerollContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DicerollContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_diceroll);
        let mut _localctx: Rc<DicerollContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(20);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(1,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule dicerollor*/
					recog.base.set_state(13);
					recog.dicerollor_rec(0)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(17);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==QUALIFIER {
						{
						/*InvokeRule qualifiers*/
						recog.base.set_state(14);
						recog.qualifiers()?;

						recog.base.set_state(15);
						recog.base.match_token(T__0,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule dicerolland*/
					recog.base.set_state(19);
					recog.dicerolland_rec(0)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- qualifiers ----------------
pub type QualifiersContextAll<'input> = QualifiersContext<'input>;


pub type QualifiersContext<'input> = BaseParserRuleContext<'input,QualifiersContextExt<'input>>;

#[derive(Clone)]
pub struct QualifiersContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DiceRollParserContext<'input> for QualifiersContext<'input>{}

impl<'input,'a> Listenable<dyn DiceRollListener<'input> + 'a> for QualifiersContext<'input>{
		fn enter(&self,listener: &mut (dyn DiceRollListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_qualifiers(self);
		}fn exit(&self,listener: &mut (dyn DiceRollListener<'input> + 'a)) {
			listener.exit_qualifiers(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for QualifiersContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DiceRollParserContextType;
	fn get_rule_index(&self) -> usize { RULE_qualifiers }
	//fn type_rule_index() -> usize where Self: Sized { RULE_qualifiers }
}
antlr_rust::tid!{QualifiersContextExt<'a>}

impl<'input> QualifiersContextExt<'input>{
	fn new(parent: Option<Rc<dyn DiceRollParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<QualifiersContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,QualifiersContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait QualifiersContextAttrs<'input>: DiceRollParserContext<'input> + BorrowMut<QualifiersContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token QUALIFIER in current rule
fn QUALIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,DiceRollParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token QUALIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token QUALIFIER is less or equal than `i`.
fn QUALIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,DiceRollParserContextType>>> where Self:Sized{
	self.get_token(QUALIFIER, i)
}

}

impl<'input> QualifiersContextAttrs<'input> for QualifiersContext<'input>{}

impl<'input, I, H> DiceRollParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn qualifiers(&mut self,)
	-> Result<Rc<QualifiersContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = QualifiersContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_qualifiers);
        let mut _localctx: Rc<QualifiersContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(23); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				recog.base.set_state(22);
				recog.base.match_token(QUALIFIER,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(25); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==QUALIFIER) {break}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- dicerolland ----------------
pub type DicerollandContextAll<'input> = DicerollandContext<'input>;


pub type DicerollandContext<'input> = BaseParserRuleContext<'input,DicerollandContextExt<'input>>;

#[derive(Clone)]
pub struct DicerollandContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DiceRollParserContext<'input> for DicerollandContext<'input>{}

impl<'input,'a> Listenable<dyn DiceRollListener<'input> + 'a> for DicerollandContext<'input>{
		fn enter(&self,listener: &mut (dyn DiceRollListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_dicerolland(self);
		}fn exit(&self,listener: &mut (dyn DiceRollListener<'input> + 'a)) {
			listener.exit_dicerolland(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for DicerollandContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DiceRollParserContextType;
	fn get_rule_index(&self) -> usize { RULE_dicerolland }
	//fn type_rule_index() -> usize where Self: Sized { RULE_dicerolland }
}
antlr_rust::tid!{DicerollandContextExt<'a>}

impl<'input> DicerollandContextExt<'input>{
	fn new(parent: Option<Rc<dyn DiceRollParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DicerollandContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DicerollandContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DicerollandContextAttrs<'input>: DiceRollParserContext<'input> + BorrowMut<DicerollandContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token NUM
/// Returns `None` if there is no child corresponding to token NUM
fn NUM(&self) -> Option<Rc<TerminalNode<'input,DiceRollParserContextType>>> where Self:Sized{
	self.get_token(NUM, 0)
}
/// Retrieves first TerminalNode corresponding to token DICE_VALUE
/// Returns `None` if there is no child corresponding to token DICE_VALUE
fn DICE_VALUE(&self) -> Option<Rc<TerminalNode<'input,DiceRollParserContextType>>> where Self:Sized{
	self.get_token(DICE_VALUE, 0)
}
fn dicerolland_all(&self) ->  Vec<Rc<DicerollandContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn dicerolland(&self, i: usize) -> Option<Rc<DicerollandContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> DicerollandContextAttrs<'input> for DicerollandContext<'input>{}

impl<'input, I, H> DiceRollParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  dicerolland(&mut self,)
	-> Result<Rc<DicerollandContextAll<'input>>,ANTLRError> {
		self.dicerolland_rec(0)
	}

	fn dicerolland_rec(&mut self, _p: isize)
	-> Result<Rc<DicerollandContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = DicerollandContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 6, RULE_dicerolland, _p);
	    let mut _localctx: Rc<DicerollandContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 6;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(34);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__2 
				=> {
					{
					recog.base.set_state(28);
					recog.base.match_token(T__2,&mut recog.err_handler)?;

					recog.base.set_state(29);
					recog.base.match_token(NUM,&mut recog.err_handler)?;

					recog.base.set_state(30);
					recog.base.match_token(T__3,&mut recog.err_handler)?;

					recog.base.set_state(31);
					recog.base.match_token(DICE_VALUE,&mut recog.err_handler)?;

					recog.base.set_state(32);
					recog.base.match_token(T__4,&mut recog.err_handler)?;

					}
				}

			 DICE_VALUE 
				=> {
					{
					recog.base.set_state(33);
					recog.base.match_token(DICE_VALUE,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(41);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(4,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = DicerollandContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_dicerolland);
					_localctx = tmp;
					recog.base.set_state(36);
					if !({recog.precpred(None, 3)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
					}
					recog.base.set_state(37);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					/*InvokeRule dicerolland*/
					recog.base.set_state(38);
					recog.dicerolland_rec(4)?;

					}
					} 
				}
				recog.base.set_state(43);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(4,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- dicerollor ----------------
pub type DicerollorContextAll<'input> = DicerollorContext<'input>;


pub type DicerollorContext<'input> = BaseParserRuleContext<'input,DicerollorContextExt<'input>>;

#[derive(Clone)]
pub struct DicerollorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DiceRollParserContext<'input> for DicerollorContext<'input>{}

impl<'input,'a> Listenable<dyn DiceRollListener<'input> + 'a> for DicerollorContext<'input>{
		fn enter(&self,listener: &mut (dyn DiceRollListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_dicerollor(self);
		}fn exit(&self,listener: &mut (dyn DiceRollListener<'input> + 'a)) {
			listener.exit_dicerollor(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for DicerollorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DiceRollParserContextType;
	fn get_rule_index(&self) -> usize { RULE_dicerollor }
	//fn type_rule_index() -> usize where Self: Sized { RULE_dicerollor }
}
antlr_rust::tid!{DicerollorContextExt<'a>}

impl<'input> DicerollorContextExt<'input>{
	fn new(parent: Option<Rc<dyn DiceRollParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DicerollorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DicerollorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DicerollorContextAttrs<'input>: DiceRollParserContext<'input> + BorrowMut<DicerollorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DICE_VALUE
/// Returns `None` if there is no child corresponding to token DICE_VALUE
fn DICE_VALUE(&self) -> Option<Rc<TerminalNode<'input,DiceRollParserContextType>>> where Self:Sized{
	self.get_token(DICE_VALUE, 0)
}
fn dicerollor_all(&self) ->  Vec<Rc<DicerollorContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn dicerollor(&self, i: usize) -> Option<Rc<DicerollorContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> DicerollorContextAttrs<'input> for DicerollorContext<'input>{}

impl<'input, I, H> DiceRollParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  dicerollor(&mut self,)
	-> Result<Rc<DicerollorContextAll<'input>>,ANTLRError> {
		self.dicerollor_rec(0)
	}

	fn dicerollor_rec(&mut self, _p: isize)
	-> Result<Rc<DicerollorContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = DicerollorContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 8, RULE_dicerollor, _p);
	    let mut _localctx: Rc<DicerollorContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 8;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			recog.base.set_state(45);
			recog.base.match_token(DICE_VALUE,&mut recog.err_handler)?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(52);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(5,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = DicerollorContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_dicerollor);
					_localctx = tmp;
					recog.base.set_state(47);
					if !({recog.precpred(None, 2)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
					}
					recog.base.set_state(48);
					recog.base.match_token(T__5,&mut recog.err_handler)?;

					/*InvokeRule dicerollor*/
					recog.base.set_state(49);
					recog.dicerollor_rec(3)?;

					}
					} 
				}
				recog.base.set_state(54);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(5,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x0c\x3a\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x03\x02\x03\x02\x03\x02\x03\x03\x03\x03\x03\x03\x03\x03\
	\x05\x03\x14\x0a\x03\x03\x03\x05\x03\x17\x0a\x03\x03\x04\x06\x04\x1a\x0a\
	\x04\x0d\x04\x0e\x04\x1b\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x05\x05\x25\x0a\x05\x03\x05\x03\x05\x03\x05\x07\x05\x2a\x0a\x05\
	\x0c\x05\x0e\x05\x2d\x0b\x05\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\
	\x06\x07\x06\x35\x0a\x06\x0c\x06\x0e\x06\x38\x0b\x06\x03\x06\x02\x04\x08\
	\x0a\x07\x02\x04\x06\x08\x0a\x02\x02\x02\x3a\x02\x0c\x03\x02\x02\x02\x04\
	\x16\x03\x02\x02\x02\x06\x19\x03\x02\x02\x02\x08\x24\x03\x02\x02\x02\x0a\
	\x2e\x03\x02\x02\x02\x0c\x0d\x05\x04\x03\x02\x0d\x0e\x07\x02\x02\x03\x0e\
	\x03\x03\x02\x02\x02\x0f\x17\x05\x0a\x06\x02\x10\x11\x05\x06\x04\x02\x11\
	\x12\x07\x03\x02\x02\x12\x14\x03\x02\x02\x02\x13\x10\x03\x02\x02\x02\x13\
	\x14\x03\x02\x02\x02\x14\x15\x03\x02\x02\x02\x15\x17\x05\x08\x05\x02\x16\
	\x0f\x03\x02\x02\x02\x16\x13\x03\x02\x02\x02\x17\x05\x03\x02\x02\x02\x18\
	\x1a\x07\x09\x02\x02\x19\x18\x03\x02\x02\x02\x1a\x1b\x03\x02\x02\x02\x1b\
	\x19\x03\x02\x02\x02\x1b\x1c\x03\x02\x02\x02\x1c\x07\x03\x02\x02\x02\x1d\
	\x1e\x08\x05\x01\x02\x1e\x1f\x07\x05\x02\x02\x1f\x20\x07\x0b\x02\x02\x20\
	\x21\x07\x06\x02\x02\x21\x22\x07\x0c\x02\x02\x22\x25\x07\x07\x02\x02\x23\
	\x25\x07\x0c\x02\x02\x24\x1d\x03\x02\x02\x02\x24\x23\x03\x02\x02\x02\x25\
	\x2b\x03\x02\x02\x02\x26\x27\x0c\x05\x02\x02\x27\x28\x07\x04\x02\x02\x28\
	\x2a\x05\x08\x05\x06\x29\x26\x03\x02\x02\x02\x2a\x2d\x03\x02\x02\x02\x2b\
	\x29\x03\x02\x02\x02\x2b\x2c\x03\x02\x02\x02\x2c\x09\x03\x02\x02\x02\x2d\
	\x2b\x03\x02\x02\x02\x2e\x2f\x08\x06\x01\x02\x2f\x30\x07\x0c\x02\x02\x30\
	\x36\x03\x02\x02\x02\x31\x32\x0c\x04\x02\x02\x32\x33\x07\x08\x02\x02\x33\
	\x35\x05\x0a\x06\x05\x34\x31\x03\x02\x02\x02\x35\x38\x03\x02\x02\x02\x36\
	\x34\x03\x02\x02\x02\x36\x37\x03\x02\x02\x02\x37\x0b\x03\x02\x02\x02\x38\
	\x36\x03\x02\x02\x02\x08\x13\x16\x1b\x24\x2b\x36";

