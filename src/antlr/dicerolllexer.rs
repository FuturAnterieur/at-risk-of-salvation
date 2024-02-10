// Generated from DiceRoll.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use antlr_rust::atn::ATN;
use antlr_rust::char_stream::CharStream;
use antlr_rust::int_stream::IntStream;
use antlr_rust::lexer::{BaseLexer, Lexer, LexerRecog};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::lexer_atn_simulator::{LexerATNSimulator, ILexerATNSimulator};
use antlr_rust::PredictionContextCache;
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::TokenSource;
use antlr_rust::token_factory::{TokenFactory,CommonTokenFactory,TokenAware};
use antlr_rust::token::*;
use antlr_rust::rule_context::{BaseRuleContext,EmptyCustomRuleContext,EmptyContext};
use antlr_rust::parser_rule_context::{ParserRuleContext,BaseParserRuleContext,cast};
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};

use antlr_rust::{lazy_static,Tid,TidAble,TidExt};

use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};


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
	pub const channelNames: [&'static str;0+2] = [
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	];

	pub const modeNames: [&'static str;1] = [
		"DEFAULT_MODE"
	];

	pub const ruleNames: [&'static str;10] = [
		"T__0", "T__1", "T__2", "T__3", "T__4", "T__5", "QUALIFIER", "NEWLINE", 
		"NUM", "DICE_VALUE"
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


pub type LexerContext<'input> = BaseRuleContext<'input,EmptyCustomRuleContext<'input,LocalTokenFactory<'input> >>;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

type From<'a> = <LocalTokenFactory<'a> as TokenFactory<'a> >::From;

pub struct DiceRollLexer<'input, Input:CharStream<From<'input> >> {
	base: BaseLexer<'input,DiceRollLexerActions,Input,LocalTokenFactory<'input>>,
}

antlr_rust::tid! { impl<'input,Input> TidAble<'input> for DiceRollLexer<'input,Input> where Input:CharStream<From<'input> > }

impl<'input, Input:CharStream<From<'input> >> Deref for DiceRollLexer<'input,Input>{
	type Target = BaseLexer<'input,DiceRollLexerActions,Input,LocalTokenFactory<'input>>;

	fn deref(&self) -> &Self::Target {
		&self.base
	}
}

impl<'input, Input:CharStream<From<'input> >> DerefMut for DiceRollLexer<'input,Input>{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.base
	}
}


impl<'input, Input:CharStream<From<'input> >> DiceRollLexer<'input,Input>{
    fn get_rule_names(&self) -> &'static [&'static str] {
        &ruleNames
    }
    fn get_literal_names(&self) -> &[Option<&str>] {
        &_LITERAL_NAMES
    }

    fn get_symbolic_names(&self) -> &[Option<&str>] {
        &_SYMBOLIC_NAMES
    }

    fn get_grammar_file_name(&self) -> &'static str {
        "DiceRollLexer.g4"
    }

	pub fn new_with_token_factory(input: Input, tf: &'input LocalTokenFactory<'input>) -> Self {
		antlr_rust::recognizer::check_version("0","3");
    	Self {
			base: BaseLexer::new_base_lexer(
				input,
				LexerATNSimulator::new_lexer_atnsimulator(
					_ATN.clone(),
					_decision_to_DFA.clone(),
					_shared_context_cache.clone(),
				),
				DiceRollLexerActions{},
				tf
			)
	    }
	}
}

impl<'input, Input:CharStream<From<'input> >> DiceRollLexer<'input,Input> where &'input LocalTokenFactory<'input>:Default{
	pub fn new(input: Input) -> Self{
		DiceRollLexer::new_with_token_factory(input, <&LocalTokenFactory<'input> as Default>::default())
	}
}

pub struct DiceRollLexerActions {
}

impl DiceRollLexerActions{
}

impl<'input, Input:CharStream<From<'input> >> Actions<'input,BaseLexer<'input,DiceRollLexerActions,Input,LocalTokenFactory<'input>>> for DiceRollLexerActions{

	fn action(_localctx: Option<&EmptyContext<'input,LocalTokenFactory<'input>> >, rule_index: isize, action_index: isize,
	          recog:&mut BaseLexer<'input,DiceRollLexerActions,Input,LocalTokenFactory<'input>>
	    ){
	    	match rule_index {
			        9 =>
			        	DiceRollLexer::<'input>::DICE_VALUE_action(None, action_index, recog), 
			_ => {}
		}
	}
	}

	impl<'input, Input:CharStream<From<'input> >> DiceRollLexer<'input,Input>{

		fn DICE_VALUE_action(_localctx: Option<&LexerContext<'input>>, action_index: isize,
						   recog:&mut <Self as Deref>::Target
			) {
			match action_index {
			 		0=>{
						1,2
					},

				_ => {}
			}
		}

}

impl<'input, Input:CharStream<From<'input> >> LexerRecog<'input,BaseLexer<'input,DiceRollLexerActions,Input,LocalTokenFactory<'input>>> for DiceRollLexerActions{
}
impl<'input> TokenAware<'input> for DiceRollLexerActions{
	type TF = LocalTokenFactory<'input>;
}

impl<'input, Input:CharStream<From<'input> >> TokenSource<'input> for DiceRollLexer<'input,Input>{
	type TF = LocalTokenFactory<'input>;

    fn next_token(&mut self) -> <Self::TF as TokenFactory<'input>>::Tok {
        self.base.next_token()
    }

    fn get_line(&self) -> isize {
        self.base.get_line()
    }

    fn get_char_position_in_line(&self) -> isize {
        self.base.get_char_position_in_line()
    }

    fn get_input_stream(&mut self) -> Option<&mut dyn IntStream> {
        self.base.get_input_stream()
    }

	fn get_source_name(&self) -> String {
		self.base.get_source_name()
	}

    fn get_token_factory(&self) -> &'input Self::TF {
        self.base.get_token_factory()
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
		"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x02\
		\x0c\x50\x08\x01\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\
		\x09\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\
		\x04\x0a\x09\x0a\x04\x0b\x09\x0b\x03\x02\x03\x02\x03\x03\x03\x03\x03\x04\
		\x03\x04\x03\x05\x03\x05\x03\x06\x03\x06\x03\x07\x03\x07\x03\x07\x03\x08\
		\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x09\x06\x09\x2c\x0a\x09\x0d\
		\x09\x0e\x09\x2d\x03\x09\x03\x09\x03\x0a\x06\x0a\x33\x0a\x0a\x0d\x0a\x0e\
		\x0a\x34\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\
		\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\
		\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x05\x0b\x4f\x0a\
		\x0b\x02\x02\x0c\x03\x03\x05\x04\x07\x05\x09\x06\x0b\x07\x0d\x08\x0f\x09\
		\x11\x0a\x13\x0b\x15\x0c\x03\x02\x04\x04\x02\x0c\x0c\x0f\x0f\x03\x02\x32\
		\x3b\x02\x57\x02\x03\x03\x02\x02\x02\x02\x05\x03\x02\x02\x02\x02\x07\x03\
		\x02\x02\x02\x02\x09\x03\x02\x02\x02\x02\x0b\x03\x02\x02\x02\x02\x0d\x03\
		\x02\x02\x02\x02\x0f\x03\x02\x02\x02\x02\x11\x03\x02\x02\x02\x02\x13\x03\
		\x02\x02\x02\x02\x15\x03\x02\x02\x02\x03\x17\x03\x02\x02\x02\x05\x19\x03\
		\x02\x02\x02\x07\x1b\x03\x02\x02\x02\x09\x1d\x03\x02\x02\x02\x0b\x1f\x03\
		\x02\x02\x02\x0d\x21\x03\x02\x02\x02\x0f\x24\x03\x02\x02\x02\x11\x2b\x03\
		\x02\x02\x02\x13\x32\x03\x02\x02\x02\x15\x4e\x03\x02\x02\x02\x17\x18\x07\
		\x3c\x02\x02\x18\x04\x03\x02\x02\x02\x19\x1a\x07\x2d\x02\x02\x1a\x06\x03\
		\x02\x02\x02\x1b\x1c\x07\x7d\x02\x02\x1c\x08\x03\x02\x02\x02\x1d\x1e\x07\
		\x7a\x02\x02\x1e\x0a\x03\x02\x02\x02\x1f\x20\x07\x7f\x02\x02\x20\x0c\x03\
		\x02\x02\x02\x21\x22\x07\x71\x02\x02\x22\x23\x07\x74\x02\x02\x23\x0e\x03\
		\x02\x02\x02\x24\x25\x07\x63\x02\x02\x25\x26\x07\x65\x02\x02\x26\x27\x07\
		\x65\x02\x02\x27\x28\x07\x77\x02\x02\x28\x29\x07\x6f\x02\x02\x29\x10\x03\
		\x02\x02\x02\x2a\x2c\x09\x02\x02\x02\x2b\x2a\x03\x02\x02\x02\x2c\x2d\x03\
		\x02\x02\x02\x2d\x2b\x03\x02\x02\x02\x2d\x2e\x03\x02\x02\x02\x2e\x2f\x03\
		\x02\x02\x02\x2f\x30\x08\x09\x02\x02\x30\x12\x03\x02\x02\x02\x31\x33\x09\
		\x03\x02\x02\x32\x31\x03\x02\x02\x02\x33\x34\x03\x02\x02\x02\x34\x32\x03\
		\x02\x02\x02\x34\x35\x03\x02\x02\x02\x35\x14\x03\x02\x02\x02\x36\x37\x09\
		\x03\x02\x02\x37\x4f\x08\x0b\x03\x02\x38\x39\x07\x51\x02\x02\x39\x3a\x07\
		\x70\x02\x02\x3a\x4f\x07\x67\x02\x02\x3b\x3c\x07\x56\x02\x02\x3c\x3d\x07\
		\x79\x02\x02\x3d\x4f\x07\x71\x02\x02\x3e\x3f\x07\x56\x02\x02\x3f\x40\x07\
		\x6a\x02\x02\x40\x41\x07\x74\x02\x02\x41\x42\x07\x67\x02\x02\x42\x4f\x07\
		\x67\x02\x02\x43\x44\x07\x48\x02\x02\x44\x45\x07\x71\x02\x02\x45\x46\x07\
		\x77\x02\x02\x46\x4f\x07\x74\x02\x02\x47\x48\x07\x48\x02\x02\x48\x49\x07\
		\x6b\x02\x02\x49\x4a\x07\x78\x02\x02\x4a\x4f\x07\x67\x02\x02\x4b\x4c\x07\
		\x55\x02\x02\x4c\x4d\x07\x6b\x02\x02\x4d\x4f\x07\x7a\x02\x02\x4e\x36\x03\
		\x02\x02\x02\x4e\x38\x03\x02\x02\x02\x4e\x3b\x03\x02\x02\x02\x4e\x3e\x03\
		\x02\x02\x02\x4e\x43\x03\x02\x02\x02\x4e\x47\x03\x02\x02\x02\x4e\x4b\x03\
		\x02\x02\x02\x4f\x16\x03\x02\x02\x02\x06\x02\x2d\x34\x4e\x04\x08\x02\x02\
		\x03\x0b\x02";
