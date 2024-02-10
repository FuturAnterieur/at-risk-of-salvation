#![allow(nonstandard_style)]
// Generated from DiceRoll.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::dicerollparser::*;

pub trait DiceRollListener<'input> : ParseTreeListener<'input,DiceRollParserContextType>{
/**
 * Enter a parse tree produced by {@link DiceRollParser#prog}.
 * @param ctx the parse tree
 */
fn enter_prog(&mut self, _ctx: &ProgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DiceRollParser#prog}.
 * @param ctx the parse tree
 */
fn exit_prog(&mut self, _ctx: &ProgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link DiceRollParser#diceroll}.
 * @param ctx the parse tree
 */
fn enter_diceroll(&mut self, _ctx: &DicerollContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DiceRollParser#diceroll}.
 * @param ctx the parse tree
 */
fn exit_diceroll(&mut self, _ctx: &DicerollContext<'input>) { }
/**
 * Enter a parse tree produced by {@link DiceRollParser#qualifiers}.
 * @param ctx the parse tree
 */
fn enter_qualifiers(&mut self, _ctx: &QualifiersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DiceRollParser#qualifiers}.
 * @param ctx the parse tree
 */
fn exit_qualifiers(&mut self, _ctx: &QualifiersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link DiceRollParser#dicerolland}.
 * @param ctx the parse tree
 */
fn enter_dicerolland(&mut self, _ctx: &DicerollandContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DiceRollParser#dicerolland}.
 * @param ctx the parse tree
 */
fn exit_dicerolland(&mut self, _ctx: &DicerollandContext<'input>) { }
/**
 * Enter a parse tree produced by {@link DiceRollParser#dicerollor}.
 * @param ctx the parse tree
 */
fn enter_dicerollor(&mut self, _ctx: &DicerollorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DiceRollParser#dicerollor}.
 * @param ctx the parse tree
 */
fn exit_dicerollor(&mut self, _ctx: &DicerollorContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : DiceRollListener<'input> }


