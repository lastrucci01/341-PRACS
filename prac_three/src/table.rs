use std::collections::HashMap;

pub struct Table {
    table: HashMap<(&'static str, &'static str), &'static str>,
}

impl Table {
    pub fn new() -> Self {
        let mut table: HashMap<(&'static str, &'static str), &'static str> = HashMap::new();

        // PROGR
        table.insert(("PROGR", "h"), "ALGO PROCDEFS");
        table.insert(("PROGR", "c"), "ALGO PROCDEFS");
        table.insert(("PROGR", "w"), "ALGO PROCDEFS");
        table.insert(("PROGR", "i"), "ALGO PROCDEFS");
        table.insert(("PROGR", "n"), "ALGO PROCDEFS");
        table.insert(("PROGR", "b"), "ALGO PROCDEFS");
        table.insert(("PROGR", "s"), "ALGO PROCDEFS");
        table.insert(("PROGR", "g"), "ALGO PROCDEFS");
        table.insert(("PROGR", "o"), "ALGO PROCDEFS");
        table.insert(("PROGR", "r"), "ALGO PROCDEFS");

        // PROCDEFS
        table.insert(("PROCDEFS", ","), ", PROC PROCDEFS");
        table.insert(("PROCDEFS", "}"), "");
        table.insert(("PROCDEFS", "$"), "");

        // PROC
        table.insert(("PROC", "p"), "p DIGITS { PROGR }");

        // DIGITS
        table.insert(("DIGITS", "0"), "D MORE");
        table.insert(("DIGITS", "1"), "D MORE");
        table.insert(("DIGITS", "2"), "D MORE");
        table.insert(("DIGITS", "3"), "D MORE");
        table.insert(("DIGITS", "4"), "D MORE");
        table.insert(("DIGITS", "5"), "D MORE");
        table.insert(("DIGITS", "6"), "D MORE");
        table.insert(("DIGITS", "7"), "D MORE");
        table.insert(("DIGITS", "8"), "D MORE");
        table.insert(("DIGITS", "9"), "D MORE");

        // D
        table.insert(("D", "0"), "0");
        table.insert(("D", "1"), "1");
        table.insert(("D", "2"), "2");
        table.insert(("D", "3"), "3");
        table.insert(("D", "4"), "4");
        table.insert(("D", "5"), "5");
        table.insert(("D", "6"), "6");
        table.insert(("D", "7"), "7");
        table.insert(("D", "8"), "8");
        table.insert(("D", "9"), "9");

        // MORE
        table.insert(("MORE", "0"), "DIGITS");
        table.insert(("MORE", "1"), "DIGITS");
        table.insert(("MORE", "2"), "DIGITS");
        table.insert(("MORE", "3"), "DIGITS");
        table.insert(("MORE", "4"), "DIGITS");
        table.insert(("MORE", "5"), "DIGITS");
        table.insert(("MORE", "6"), "DIGITS");
        table.insert(("MORE", "7"), "DIGITS");
        table.insert(("MORE", "8"), "DIGITS");
        table.insert(("MORE", "9"), "DIGITS");
        table.insert(("MORE", ","), "");
        table.insert(("MORE", "{"), "");
        table.insert(("MORE", "}"), "");
        table.insert(("MORE", ";"), "");
        table.insert(("MORE", ":="), "");
        table.insert(("MORE", ")"), "");
        table.insert(("MORE", "."), "");
        table.insert(("MORE", "*"), "");
        table.insert(("MORE", "$"), "");

        // ALGO
        table.insert(("ALGO", "h"), "INSTR COMMENT SEQ");
        table.insert(("ALGO", "c"), "INSTR COMMENT SEQ");
        table.insert(("ALGO", "w"), "INSTR COMMENT SEQ");
        table.insert(("ALGO", "i"), "INSTR COMMENT SEQ");
        table.insert(("ALGO", "n"), "INSTR COMMENT SEQ");
        table.insert(("ALGO", "b"), "INSTR COMMENT SEQ");
        table.insert(("ALGO", "s"), "INSTR COMMENT SEQ");
        table.insert(("ALGO", "g"), "INSTR COMMENT SEQ");
        table.insert(("ALGO", "o"), "INSTR COMMENT SEQ");
        table.insert(("ALGO", "r"), "INSTR COMMENT SEQ");

        // SEQ
        table.insert(("SEQ", ";"), "; ALGO");
        table.insert(("SEQ", ","), "");
        table.insert(("SEQ", "}"), "");
        table.insert(("SEQ", "$"), "");

        // INSTR
        table.insert(("INSTR", "h"), "h");
        table.insert(("INSTR", "c"), "CALL");
        table.insert(("INSTR", "w"), "LOOP");
        table.insert(("INSTR", "i"), "BRANCH");
        table.insert(("INSTR", "n"), "ASSIGN");
        table.insert(("INSTR", "b"), "ASSIGN");
        table.insert(("INSTR", "s"), "ASSIGN");
        table.insert(("INSTR", "g"), "INPUT");
        table.insert(("INSTR", "o"), "OUTPUT");
        table.insert(("INSTR", "r"), "OUTPUT");

        // CALL
        table.insert(("CALL", "c"), "c p DIGITS");

        // ASSIGN
        table.insert(("ASSIGN", "n"), "NUMVAR := NUMEXPR");
        table.insert(("ASSIGN", "b"), "BOOLVAR := BOOLEXPR");
        table.insert(("ASSIGN", "s"), "STRINGV := STRI");

        // LOOP
        table.insert(("LOOP", "w"), "w ( BOOLEXPR ) { ALGO }");

        // BRANCH
        table.insert(("BRANCH", "i"), "i ( BOOLEXPR ) t { ALGO } ELSE");

        // ELSE
        table.insert(("ELSE", "e"), "e { ALGO }");
        table.insert(("ELSE", ","), "");
        table.insert(("ELSE", "}"), "");
        table.insert(("ELSE", ";"), "");
        table.insert(("ELSE", "*"), "");
        table.insert(("ELSE", "$"), "");

        // NUMVAR
        table.insert(("NUMVAR", "n"), "n DIGITS");

        // BOOLVAR
        table.insert(("BOOLVAR", "b"), "b DIGITS");

        // STRINGV
        table.insert(("STRINGV", "s"), "s DIGITS");

        // NUMEXPR
        table.insert(("NUMEXPR", "1"), "DECNUM");
        table.insert(("NUMEXPR", "2"), "DECNUM");
        table.insert(("NUMEXPR", "3"), "DECNUM");
        table.insert(("NUMEXPR", "4"), "DECNUM");
        table.insert(("NUMEXPR", "5"), "DECNUM");
        table.insert(("NUMEXPR", "6"), "DECNUM");
        table.insert(("NUMEXPR", "7"), "DECNUM");
        table.insert(("NUMEXPR", "8"), "DECNUM");
        table.insert(("NUMEXPR", "9"), "DECNUM");
        table.insert(("NUMEXPR", "0.00"), "DECNUM");
        table.insert(("NUMEXPR", "-"), "DECNUM");
        table.insert(("NUMEXPR", "n"), "NUMVAR");
        table.insert(("NUMEXPR", "a"), "a ( NUMEXPR , NUMEXPR )	");
        table.insert(("NUMEXPR", "m"), "m ( NUMEXPR , NUMEXPR )");
        table.insert(("NUMEXPR", "d"), "d ( NUMEXPR , NUMEXPR )");
        table.insert(("NUMEXPR", "d"), "d ( NUMEXPR , NUMEXPR )");

        // DECNUM
        table.insert(("DECNUM", "1"), "POS");
        table.insert(("DECNUM", "2"), "POS");
        table.insert(("DECNUM", "3"), "POS");
        table.insert(("DECNUM", "4"), "POS");
        table.insert(("DECNUM", "5"), "POS");
        table.insert(("DECNUM", "6"), "POS");
        table.insert(("DECNUM", "7"), "POS");
        table.insert(("DECNUM", "8"), "POS");
        table.insert(("DECNUM", "9"), "POS");
        table.insert(("DECNUM", "0.00"), "0.00");
        table.insert(("DECNUM", "-"), "NEG");

        // NEG
        table.insert(("NEG", "-"), "- POS");

        // POS
        table.insert(("POS", "1"), "INT . D D");
        table.insert(("POS", "2"), "INT . D D");
        table.insert(("POS", "3"), "INT . D D");
        table.insert(("POS", "4"), "INT . D D");
        table.insert(("POS", "5"), "INT . D D");
        table.insert(("POS", "6"), "INT . D D");
        table.insert(("POS", "7"), "INT . D D");
        table.insert(("POS", "8"), "INT . D D");
        table.insert(("POS", "9"), "INT . D D");

        // INT
        table.insert(("INT", "1"), "1 MORE");
        table.insert(("INT", "2"), "2 MORE");
        table.insert(("INT", "3"), "3 MORE");
        table.insert(("INT", "4"), "4 MORE");
        table.insert(("INT", "5"), "5 MORE");
        table.insert(("INT", "6"), "6 MORE");
        table.insert(("INT", "7"), "7 MORE");
        table.insert(("INT", "8"), "8 MORE");
        table.insert(("INT", "9"), "9 MORE");

        // BOOLEXPR
        table.insert(("BOOLEXPR", "b"), "LOGIC");
        table.insert(("BOOLEXPR", "T"), "LOGIC");
        table.insert(("BOOLEXPR", "F"), "LOGIC");
        table.insert(("BOOLEXPR", "^"), "LOGIC");
        table.insert(("BOOLEXPR", "v"), "LOGIC");
        table.insert(("BOOLEXPR", "!"), "LOGIC");
        table.insert(("BOOLEXPR", "E"), "CMPR");
        table.insert(("BOOLEXPR", "<"), "CMPR");
        table.insert(("BOOLEXPR", ">"), "CMPR");

        // LOGIC
        table.insert(("LOGIC", "b"), "BOOLVAR");
        table.insert(("LOGIC", "T"), "T");
        table.insert(("LOGIC", "F"), "F");
        table.insert(("LOGIC", "^"), "^ ( BOOLEXPR , BOOLEXPR )");
        table.insert(("LOGIC", "v"), "v ( BOOLEXPR , BOOLEXPR )");
        table.insert(("LOGIC", "!"), "! ( BOOLEXPR )");

        // CMPR
        table.insert(("CMPR", "E"), "E ( NUMEXPR , NUMEXPR )");
        table.insert(("CMPR", "<"), "< ( NUMEXPR , NUMEXPR )");
        table.insert(("CMPR", ">"), "> ( NUMEXPR , NUMEXPR )");

        // STRI
        table.insert(("STRI", "\""), "\" C C C C C C C C C C C C C C C \"");

        // COMMENT
        table.insert(("COMMENT", "*"), "* C C C C C C C C C C C C C C C *");

        table.insert(("COMMENT", ","), "");
        table.insert(("COMMENT", "}"), "");
        table.insert(("COMMENT", ";"), "");
        table.insert(("COMMENT", "$"), "");

        // C

        // INPUT
        table.insert(("INPUT", "g"), "g NUMVAR");

        // OUTPUT
        table.insert(("OUTPUT", "o"), "VALUE");
        table.insert(("OUTPUT", "r"), "TEXT");

        // VALUE
        table.insert(("VALUE", "o"), "o NUMVAR");

        // TEXT
        table.insert(("TEXT", "r"), "r STRINGV");

        Self { table }
    }

    pub fn get_from_table(&self, non_term: &str, term: &str) -> Option<&'static str> {
        let val = self.table.get(&(non_term, term));

        if let Some(val) = val {
            Some(*val)
        } else {
            None
        }
    }
}
