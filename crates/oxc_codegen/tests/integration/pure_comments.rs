use oxc_allocator::Allocator;
use oxc_codegen::{CodeGenerator, CommentOptions};
use oxc_parser::Parser;
use oxc_span::SourceType;

fn test_comment_helper(source_text: &str, expected: &str) {
    let allocator = Allocator::default();
    let source_type = SourceType::default().with_module(true);
    let ret = Parser::new(&allocator, source_text, source_type).parse();
    let result = CodeGenerator::new()
        .enable_comment(
            source_text,
            ret.trivias,
            CommentOptions { preserve_annotate_comments: true },
        )
        .build(&ret.program)
        .source_text;
    assert_eq!(expected, result, "for source {source_text}, expect {expected}, got {result}");
}

#[test]
fn annotate_comment() {
    test_comment_helper(
        r"
    				x([
    					/* #__NO_SIDE_EFFECTS__ */ function() {},
    					/* #__NO_SIDE_EFFECTS__ */ function y() {},
    					/* #__NO_SIDE_EFFECTS__ */ function*() {},
    					/* #__NO_SIDE_EFFECTS__ */ function* y() {},
    					/* #__NO_SIDE_EFFECTS__ */ async function() {},
    					/* #__NO_SIDE_EFFECTS__ */ async function y() {},
    					/* #__NO_SIDE_EFFECTS__ */ async function*() {},
    					/* #__NO_SIDE_EFFECTS__ */ async function* y() {},
    				])",
        r"x([/* #__NO_SIDE_EFFECTS__ */ function() {}, /* #__NO_SIDE_EFFECTS__ */ function y() {}, /* #__NO_SIDE_EFFECTS__ */ function* () {}, /* #__NO_SIDE_EFFECTS__ */ function* y() {}, /* #__NO_SIDE_EFFECTS__ */ async function() {}, /* #__NO_SIDE_EFFECTS__ */ async function y() {}, /* #__NO_SIDE_EFFECTS__ */ async function* () {}, /* #__NO_SIDE_EFFECTS__ */ async function* y() {},]);
",
    );

    test_comment_helper(
        r"
            x([
    					/* #__NO_SIDE_EFFECTS__ */ y => y,
    					/* #__NO_SIDE_EFFECTS__ */ () => {},
    					/* #__NO_SIDE_EFFECTS__ */ (y) => (y),
    					/* #__NO_SIDE_EFFECTS__ */ async y => y,
    					/* #__NO_SIDE_EFFECTS__ */ async () => {},
    					/* #__NO_SIDE_EFFECTS__ */ async (y) => (y),
    				])",
        r"x([/* #__NO_SIDE_EFFECTS__ */ (y) => y, /* #__NO_SIDE_EFFECTS__ */ () => {}, /* #__NO_SIDE_EFFECTS__ */ (y) => y, /* #__NO_SIDE_EFFECTS__ */ async (y) => y, /* #__NO_SIDE_EFFECTS__ */ async () => {}, /* #__NO_SIDE_EFFECTS__ */ async (y) => y,]);
",
    );
    test_comment_helper(
        r"
            x([
    					/* #__NO_SIDE_EFFECTS__ */ y => y,
    					/* #__NO_SIDE_EFFECTS__ */ () => {},
    					/* #__NO_SIDE_EFFECTS__ */ (y) => (y),
    					/* #__NO_SIDE_EFFECTS__ */ async y => y,
    					/* #__NO_SIDE_EFFECTS__ */ async () => {},
    					/* #__NO_SIDE_EFFECTS__ */ async (y) => (y),
    				])",
        r"x([/* #__NO_SIDE_EFFECTS__ */ (y) => y, /* #__NO_SIDE_EFFECTS__ */ () => {}, /* #__NO_SIDE_EFFECTS__ */ (y) => y, /* #__NO_SIDE_EFFECTS__ */ async (y) => y, /* #__NO_SIDE_EFFECTS__ */ async () => {}, /* #__NO_SIDE_EFFECTS__ */ async (y) => y,]);
",
    );
    //
    test_comment_helper(
        r"
    // #__NO_SIDE_EFFECTS__
    function a() {}
    // #__NO_SIDE_EFFECTS__
    function* b() {}
    // #__NO_SIDE_EFFECTS__
    async function c() {}
    // #__NO_SIDE_EFFECTS__
    async function* d() {}
            ",
        r"// #__NO_SIDE_EFFECTS__
function a() {}
// #__NO_SIDE_EFFECTS__
function* b() {}
// #__NO_SIDE_EFFECTS__
async function c() {}
// #__NO_SIDE_EFFECTS__
async function* d() {}
",
    );

    test_comment_helper(
        r"
    // #__NO_SIDE_EFFECTS__
    function a() {}
    // #__NO_SIDE_EFFECTS__
    function* b() {}
    // #__NO_SIDE_EFFECTS__
    async function c() {}
    // #__NO_SIDE_EFFECTS__
    async function* d() {}
            ",
        r"// #__NO_SIDE_EFFECTS__
function a() {}
// #__NO_SIDE_EFFECTS__
function* b() {}
// #__NO_SIDE_EFFECTS__
async function c() {}
// #__NO_SIDE_EFFECTS__
async function* d() {}
",
    );

    test_comment_helper(
        r"
    /* @__NO_SIDE_EFFECTS__ */ export function a() {}
    /* @__NO_SIDE_EFFECTS__ */ export function* b() {}
    /* @__NO_SIDE_EFFECTS__ */ export async function c() {}
    /* @__NO_SIDE_EFFECTS__ */ export async function* d() {}",
        r"/* @__NO_SIDE_EFFECTS__ */ export function a() {}
/* @__NO_SIDE_EFFECTS__ */ export function* b() {}
/* @__NO_SIDE_EFFECTS__ */ export async function c() {}
/* @__NO_SIDE_EFFECTS__ */ export async function* d() {}
",
    );
    // Only "c0" and "c2" should have "no side effects" (Rollup only respects "const" and only for the first one)
    test_comment_helper(
        r"
        					/* #__NO_SIDE_EFFECTS__ */ export var v0 = function() {}, v1 = function() {}
        					/* #__NO_SIDE_EFFECTS__ */ export let l0 = function() {}, l1 = function() {}
        					/* #__NO_SIDE_EFFECTS__ */ export const c0 = function() {}, c1 = function() {}
        					/* #__NO_SIDE_EFFECTS__ */ export var v2 = () => {}, v3 = () => {}
        					/* #__NO_SIDE_EFFECTS__ */ export let l2 = () => {}, l3 = () => {}
        					/* #__NO_SIDE_EFFECTS__ */ export const c2 = () => {}, c3 = () => {}
        ",
        r"export var v0 = function() {}, v1 = function() {};
export let l0 = function() {}, l1 = function() {};
export const c0 = /* #__NO_SIDE_EFFECTS__ */ function() {}, c1 = function() {};
export var v2 = () => {}, v3 = () => {};
export let l2 = () => {}, l3 = () => {};
export const c2 = /* #__NO_SIDE_EFFECTS__ */ () => {}, c3 = () => {};
",
    );
    // Only "c0" and "c2" should have "no side effects" (Rollup only respects "const" and only for the first one)
    test_comment_helper(
        r"
    /* #__NO_SIDE_EFFECTS__ */ var v0 = function() {}, v1 = function() {}
    /* #__NO_SIDE_EFFECTS__ */ let l0 = function() {}, l1 = function() {}
    /* #__NO_SIDE_EFFECTS__ */ const c0 = function() {}, c1 = function() {}
    /* #__NO_SIDE_EFFECTS__ */ var v2 = () => {}, v3 = () => {}
    /* #__NO_SIDE_EFFECTS__ */ let l2 = () => {}, l3 = () => {}
    /* #__NO_SIDE_EFFECTS__ */ const c2 = () => {}, c3 = () => {}
        ",
        r"var v0 = function() {}, v1 = function() {};
let l0 = function() {}, l1 = function() {};
const c0 = /* #__NO_SIDE_EFFECTS__ */ function() {}, c1 = function() {};
var v2 = () => {}, v3 = () => {};
let l2 = () => {}, l3 = () => {};
const c2 = /* #__NO_SIDE_EFFECTS__ */ () => {}, c3 = () => {};
",
    );

    test_comment_helper(
        r"
isFunction(options)
    ? // #8326: extend call and options.name access are considered side-effects
      // by Rollup, so we have to wrap it in a pure-annotated IIFE.
      /*#__PURE__*/ (() =>
        extend({ name: options.name }, extraOptions, { setup: options }))()
    : options
        ",
        r"isFunction(options) ? /*#__PURE__*/ (() => extend({name: options.name}, extraOptions, {setup: options}))() : options;
",
    );

    test_comment_helper(
        r"
const obj = {
  props: /*#__PURE__*/ extend({}, TransitionPropsValidators, {
    tag: String,
    moveClass: String,
  }),
};
const p = /*#__PURE__*/ Promise.resolve();
",
        "const obj = {props: /*#__PURE__*/ extend({}, TransitionPropsValidators, {\n\ttag: String,\n\tmoveClass: String\n})};\nconst p = /*#__PURE__*/ Promise.resolve();\n",
    );

    test_comment_helper(
        r"
const staticCacheMap = /*#__PURE__*/ new WeakMap()
",
        "const staticCacheMap = /*#__PURE__*/ new WeakMap();\n",
    );

    test_comment_helper(
        r"
const builtInSymbols = new Set(
  /*#__PURE__*/
  Object.getOwnPropertyNames(Symbol)
    .filter(key => key !== 'arguments' && key !== 'caller')
)

",
        "const builtInSymbols = new Set(/*#__PURE__*/ (Object.getOwnPropertyNames(Symbol)).filter((key) => key !== \"arguments\" && key !== \"caller\"));\n",
    );
}
