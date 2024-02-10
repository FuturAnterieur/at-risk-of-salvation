grammar DiceRoll;		
prog:	diceroll EOF;


diceroll:	    dicerollor
            |   (qualifiers ':')? dicerolland
            ;

qualifiers: QUALIFIER+;

dicerolland:             dicerolland '+' dicerolland
                    |   '{' NUM 'x' DICE_VALUE '}'
                    |	 DICE_VALUE
                    ;

dicerollor:          dicerollor 'or' dicerollor
                    |  DICE_VALUE
                    ;

QUALIFIER : 'accum';
NEWLINE : [\r\n]+ -> skip;
NUM     : [0-9]+ ;
DICE_VALUE : [0-9]{1,2} | 'One' | 'Two' | 'Three' | 'Four' | 'Five' | 'Six';
