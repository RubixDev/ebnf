#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 27
#define LARGE_STATE_COUNT 7
#define SYMBOL_COUNT 25
#define ALIAS_COUNT 0
#define TOKEN_COUNT 18
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 5
#define MAX_ALIAS_SEQUENCE_LENGTH 4
#define PRODUCTION_ID_COUNT 5

enum {
  sym_terminal = 1,
  sym_identifier = 2,
  sym_integer = 3,
  sym_comment = 4,
  sym_special_sequence = 5,
  anon_sym_EQ = 6,
  anon_sym_SEMI = 7,
  anon_sym_PIPE = 8,
  anon_sym_COMMA = 9,
  anon_sym_DASH = 10,
  anon_sym_STAR = 11,
  anon_sym_LBRACK = 12,
  anon_sym_RBRACK = 13,
  anon_sym_LBRACE = 14,
  anon_sym_RBRACE = 15,
  anon_sym_LPAREN = 16,
  anon_sym_RPAREN = 17,
  sym_syntax = 18,
  sym_syntax_rule = 19,
  sym__expression = 20,
  sym__atom = 21,
  sym_binary_expression = 22,
  sym_group = 23,
  aux_sym_syntax_repeat1 = 24,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_terminal] = "terminal",
  [sym_identifier] = "identifier",
  [sym_integer] = "integer",
  [sym_comment] = "comment",
  [sym_special_sequence] = "special_sequence",
  [anon_sym_EQ] = "=",
  [anon_sym_SEMI] = ";",
  [anon_sym_PIPE] = "|",
  [anon_sym_COMMA] = ",",
  [anon_sym_DASH] = "-",
  [anon_sym_STAR] = "*",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [sym_syntax] = "syntax",
  [sym_syntax_rule] = "syntax_rule",
  [sym__expression] = "_expression",
  [sym__atom] = "_atom",
  [sym_binary_expression] = "binary_expression",
  [sym_group] = "group",
  [aux_sym_syntax_repeat1] = "syntax_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_terminal] = sym_terminal,
  [sym_identifier] = sym_identifier,
  [sym_integer] = sym_integer,
  [sym_comment] = sym_comment,
  [sym_special_sequence] = sym_special_sequence,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [anon_sym_PIPE] = anon_sym_PIPE,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_DASH] = anon_sym_DASH,
  [anon_sym_STAR] = anon_sym_STAR,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [sym_syntax] = sym_syntax,
  [sym_syntax_rule] = sym_syntax_rule,
  [sym__expression] = sym__expression,
  [sym__atom] = sym__atom,
  [sym_binary_expression] = sym_binary_expression,
  [sym_group] = sym_group,
  [aux_sym_syntax_repeat1] = aux_sym_syntax_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_terminal] = {
    .visible = true,
    .named = true,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_integer] = {
    .visible = true,
    .named = true,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_special_sequence] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PIPE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_STAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [sym_syntax] = {
    .visible = true,
    .named = true,
  },
  [sym_syntax_rule] = {
    .visible = true,
    .named = true,
  },
  [sym__expression] = {
    .visible = false,
    .named = true,
  },
  [sym__atom] = {
    .visible = false,
    .named = true,
  },
  [sym_binary_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_group] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_syntax_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum {
  field_definition = 1,
  field_left = 2,
  field_name = 3,
  field_operator = 4,
  field_right = 5,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_definition] = "definition",
  [field_left] = "left",
  [field_name] = "name",
  [field_operator] = "operator",
  [field_right] = "right",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 2},
  [3] = {.index = 3, .length = 2},
  [4] = {.index = 5, .length = 3},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_name, 0},
  [1] =
    {field_definition, 2},
    {field_name, 0},
  [3] =
    {field_left, 0},
    {field_operator, 1},
  [5] =
    {field_left, 0},
    {field_operator, 1},
    {field_right, 2},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(6);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (lookahead == '"') ADVANCE(1);
      if (lookahead == '\'') ADVANCE(2);
      if (lookahead == '(') ADVANCE(22);
      if (lookahead == ')') ADVANCE(23);
      if (lookahead == '*') ADVANCE(17);
      if (lookahead == ',') ADVANCE(15);
      if (lookahead == '-') ADVANCE(16);
      if (lookahead == ';') ADVANCE(13);
      if (lookahead == '=') ADVANCE(12);
      if (lookahead == '?') ADVANCE(5);
      if (lookahead == '[') ADVANCE(18);
      if (lookahead == ']') ADVANCE(19);
      if (lookahead == '{') ADVANCE(20);
      if (lookahead == '|') ADVANCE(14);
      if (lookahead == '}') ADVANCE(21);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(9);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(8);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(7);
      if (lookahead != 0) ADVANCE(1);
      END_STATE();
    case 2:
      if (lookahead == '\'') ADVANCE(7);
      if (lookahead != 0) ADVANCE(2);
      END_STATE();
    case 3:
      if (lookahead == ')') ADVANCE(10);
      if (lookahead == '*') ADVANCE(3);
      if (lookahead != 0) ADVANCE(4);
      END_STATE();
    case 4:
      if (lookahead == '*') ADVANCE(3);
      if (lookahead != 0) ADVANCE(4);
      END_STATE();
    case 5:
      if (lookahead == '?') ADVANCE(11);
      if (lookahead != 0) ADVANCE(5);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(sym_terminal);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(8);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(sym_integer);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(9);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(sym_comment);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(sym_special_sequence);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      if (lookahead == '*') ADVANCE(4);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 0},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 0},
  [25] = {.lex_state = 0},
  [26] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_terminal] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [sym_integer] = ACTIONS(1),
    [sym_comment] = ACTIONS(3),
    [sym_special_sequence] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [anon_sym_PIPE] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [anon_sym_STAR] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
  },
  [1] = {
    [sym_syntax] = STATE(26),
    [sym_syntax_rule] = STATE(21),
    [aux_sym_syntax_repeat1] = STATE(21),
    [sym_identifier] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
  },
  [2] = {
    [sym__expression] = STATE(10),
    [sym__atom] = STATE(10),
    [sym_binary_expression] = STATE(10),
    [sym_group] = STATE(10),
    [sym_terminal] = ACTIONS(7),
    [sym_identifier] = ACTIONS(7),
    [sym_integer] = ACTIONS(9),
    [sym_comment] = ACTIONS(3),
    [sym_special_sequence] = ACTIONS(7),
    [anon_sym_SEMI] = ACTIONS(11),
    [anon_sym_PIPE] = ACTIONS(11),
    [anon_sym_COMMA] = ACTIONS(11),
    [anon_sym_DASH] = ACTIONS(11),
    [anon_sym_LBRACK] = ACTIONS(13),
    [anon_sym_RBRACK] = ACTIONS(11),
    [anon_sym_LBRACE] = ACTIONS(15),
    [anon_sym_RBRACE] = ACTIONS(11),
    [anon_sym_LPAREN] = ACTIONS(17),
    [anon_sym_RPAREN] = ACTIONS(11),
  },
  [3] = {
    [sym__expression] = STATE(16),
    [sym__atom] = STATE(16),
    [sym_binary_expression] = STATE(16),
    [sym_group] = STATE(16),
    [sym_terminal] = ACTIONS(19),
    [sym_identifier] = ACTIONS(19),
    [sym_integer] = ACTIONS(9),
    [sym_comment] = ACTIONS(3),
    [sym_special_sequence] = ACTIONS(19),
    [anon_sym_SEMI] = ACTIONS(21),
    [anon_sym_LBRACK] = ACTIONS(13),
    [anon_sym_LBRACE] = ACTIONS(15),
    [anon_sym_LPAREN] = ACTIONS(17),
  },
  [4] = {
    [sym__expression] = STATE(18),
    [sym__atom] = STATE(18),
    [sym_binary_expression] = STATE(18),
    [sym_group] = STATE(18),
    [sym_terminal] = ACTIONS(23),
    [sym_identifier] = ACTIONS(23),
    [sym_integer] = ACTIONS(9),
    [sym_comment] = ACTIONS(3),
    [sym_special_sequence] = ACTIONS(23),
    [anon_sym_LBRACK] = ACTIONS(13),
    [anon_sym_RBRACK] = ACTIONS(25),
    [anon_sym_LBRACE] = ACTIONS(15),
    [anon_sym_LPAREN] = ACTIONS(17),
  },
  [5] = {
    [sym__expression] = STATE(19),
    [sym__atom] = STATE(19),
    [sym_binary_expression] = STATE(19),
    [sym_group] = STATE(19),
    [sym_terminal] = ACTIONS(27),
    [sym_identifier] = ACTIONS(27),
    [sym_integer] = ACTIONS(9),
    [sym_comment] = ACTIONS(3),
    [sym_special_sequence] = ACTIONS(27),
    [anon_sym_LBRACK] = ACTIONS(13),
    [anon_sym_LBRACE] = ACTIONS(15),
    [anon_sym_RBRACE] = ACTIONS(25),
    [anon_sym_LPAREN] = ACTIONS(17),
  },
  [6] = {
    [sym__expression] = STATE(17),
    [sym__atom] = STATE(17),
    [sym_binary_expression] = STATE(17),
    [sym_group] = STATE(17),
    [sym_terminal] = ACTIONS(29),
    [sym_identifier] = ACTIONS(29),
    [sym_integer] = ACTIONS(9),
    [sym_comment] = ACTIONS(3),
    [sym_special_sequence] = ACTIONS(29),
    [anon_sym_LBRACK] = ACTIONS(13),
    [anon_sym_LBRACE] = ACTIONS(15),
    [anon_sym_LPAREN] = ACTIONS(17),
    [anon_sym_RPAREN] = ACTIONS(25),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_integer,
    ACTIONS(13), 1,
      anon_sym_LBRACK,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(31), 3,
      sym_terminal,
      sym_identifier,
      sym_special_sequence,
    STATE(11), 4,
      sym__expression,
      sym__atom,
      sym_binary_expression,
      sym_group,
  [27] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_integer,
    ACTIONS(13), 1,
      anon_sym_LBRACK,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(33), 3,
      sym_terminal,
      sym_identifier,
      sym_special_sequence,
    STATE(14), 4,
      sym__expression,
      sym__atom,
      sym_binary_expression,
      sym_group,
  [54] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_integer,
    ACTIONS(13), 1,
      anon_sym_LBRACK,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 3,
      sym_terminal,
      sym_identifier,
      sym_special_sequence,
    STATE(12), 4,
      sym__expression,
      sym__atom,
      sym_binary_expression,
      sym_group,
  [81] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(37), 7,
      anon_sym_SEMI,
      anon_sym_PIPE,
      anon_sym_COMMA,
      anon_sym_DASH,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
  [94] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(39), 1,
      anon_sym_DASH,
    ACTIONS(37), 6,
      anon_sym_SEMI,
      anon_sym_PIPE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
  [109] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(39), 1,
      anon_sym_DASH,
    ACTIONS(41), 1,
      anon_sym_COMMA,
    ACTIONS(37), 5,
      anon_sym_SEMI,
      anon_sym_PIPE,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
  [126] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(43), 7,
      anon_sym_SEMI,
      anon_sym_PIPE,
      anon_sym_COMMA,
      anon_sym_DASH,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
  [139] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(37), 7,
      anon_sym_SEMI,
      anon_sym_PIPE,
      anon_sym_COMMA,
      anon_sym_DASH,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
  [152] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(45), 7,
      anon_sym_SEMI,
      anon_sym_PIPE,
      anon_sym_COMMA,
      anon_sym_DASH,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
  [165] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(39), 1,
      anon_sym_DASH,
    ACTIONS(41), 1,
      anon_sym_COMMA,
    ACTIONS(47), 1,
      anon_sym_SEMI,
    ACTIONS(49), 1,
      anon_sym_PIPE,
  [181] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(39), 1,
      anon_sym_DASH,
    ACTIONS(41), 1,
      anon_sym_COMMA,
    ACTIONS(49), 1,
      anon_sym_PIPE,
    ACTIONS(51), 1,
      anon_sym_RPAREN,
  [197] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(39), 1,
      anon_sym_DASH,
    ACTIONS(41), 1,
      anon_sym_COMMA,
    ACTIONS(49), 1,
      anon_sym_PIPE,
    ACTIONS(51), 1,
      anon_sym_RBRACK,
  [213] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(39), 1,
      anon_sym_DASH,
    ACTIONS(41), 1,
      anon_sym_COMMA,
    ACTIONS(49), 1,
      anon_sym_PIPE,
    ACTIONS(51), 1,
      anon_sym_RBRACE,
  [229] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(53), 1,
      ts_builtin_sym_end,
    ACTIONS(55), 1,
      sym_identifier,
    STATE(20), 2,
      sym_syntax_rule,
      aux_sym_syntax_repeat1,
  [243] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(5), 1,
      sym_identifier,
    ACTIONS(58), 1,
      ts_builtin_sym_end,
    STATE(20), 2,
      sym_syntax_rule,
      aux_sym_syntax_repeat1,
  [257] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(60), 2,
      ts_builtin_sym_end,
      sym_identifier,
  [265] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(62), 2,
      ts_builtin_sym_end,
      sym_identifier,
  [273] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(64), 1,
      anon_sym_EQ,
  [280] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(66), 1,
      anon_sym_STAR,
  [287] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(68), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(7)] = 0,
  [SMALL_STATE(8)] = 27,
  [SMALL_STATE(9)] = 54,
  [SMALL_STATE(10)] = 81,
  [SMALL_STATE(11)] = 94,
  [SMALL_STATE(12)] = 109,
  [SMALL_STATE(13)] = 126,
  [SMALL_STATE(14)] = 139,
  [SMALL_STATE(15)] = 152,
  [SMALL_STATE(16)] = 165,
  [SMALL_STATE(17)] = 181,
  [SMALL_STATE(18)] = 197,
  [SMALL_STATE(19)] = 213,
  [SMALL_STATE(20)] = 229,
  [SMALL_STATE(21)] = 243,
  [SMALL_STATE(22)] = 257,
  [SMALL_STATE(23)] = 265,
  [SMALL_STATE(24)] = 273,
  [SMALL_STATE(25)] = 280,
  [SMALL_STATE(26)] = 287,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [11] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_expression, 2, .production_id = 3),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [17] = {.entry = {.count = 1, .reusable = false}}, SHIFT(6),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [37] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_expression, 3, .production_id = 4),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [43] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_group, 3),
  [45] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_group, 2),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [49] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [53] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_syntax_repeat1, 2),
  [55] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_syntax_repeat1, 2), SHIFT_REPEAT(24),
  [58] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_syntax, 1),
  [60] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_syntax_rule, 4, .production_id = 2),
  [62] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_syntax_rule, 3, .production_id = 1),
  [64] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [66] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [68] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_ebnf(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
