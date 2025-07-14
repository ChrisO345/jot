/**
 * @file Grammar definition for the Jot language using Tree-sitter
 */

const colon = ':';
const semicolon = ';';
const equal = '=';
const dot = '.';
const newline = /\r?\n/;
const reference = /@/;

module.exports = grammar({
  name: "jot",

  rules: {
    source_file: $ => repeat($._definition),

    _definition: $ => choice(
      $.command_definition,
      $.section_definition,
      $.variable_definition,
    ),

    // COMMANDS
    command_definition: $ => seq(
      $.command_name,
      colon,
      $.command_body,
    ),

    command_name: $ => $.identifier,

    command_body: $ => seq(
      choice(
        $._inline_command_body,
        $._overline_command_body,
      ),
      newline,
    ),

    _inline_command_body: $ => seq(
      $.command,
      optional(semicolon),
    ),

    _overline_command_body: $ => seq(
      repeat(
        seq(
          newline,
          $.command,
        ),
      ),
      semicolon,
    ),

    // SECTIONS
    section_definition: $ => seq(
      equal,
      $.section_name,
      newline,
    ),

    section_name: $ => choice(
      $.identifier,
      $.string,
    ),


    // VARIABLES
    variable_definition: $ => seq(
      dot,
      $.variable_name,
      equal,
      $.variable_value,
      newline,
    ),

    variable_name: $ => $.identifier,
    variable_value: $ => choice(
      $.string,
      $._cmd,
    ),

    // COMMENT
    comment: _ => token(seq('#', /.*/)),

    // TERMINALS
    command: $ => seq(
      optional($.reference),
      $._cmd,
    ),

    _cmd: $ => repeat1(/[^\s\n\r;#@]+/,),

    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_]*/,

    string: $ => seq(
      '"',
      repeat(choice(
        /[^"\\\n]+/,
        seq('\\', /./),
      )),
      '"',
    ),

    reference: $ => reference,
  },

  extras: $ => [
    /[ \t\r]+/, // whitespace
    $.comment,  // allow comments anywhere
    newline     // treat newlines as insignificant where allowed
  ],
});
