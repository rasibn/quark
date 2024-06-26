use anyhow::{bail, Result};
use std::iter::Peekable;

use super::*;
use crate::compiler::Error;
use crate::language::lexicon::token::Kind::*;

use crate::language::semantics::r#type::Type;
use crate::language::{grammar::FunctionDclr, lexicon::Token, utils::Span};

impl FunctionDclr
{
	/// ### Prior:
	/// has seen a Declaration token
	pub fn try_from_stream<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Result<Self>
	where
		I: Iterator<Item = Token>,
	{
		let fn_token = stream.next().expect("fn Token");
		let start = fn_token.span.start;
		let mut end = fn_token.span.end;
		let option = stream.next();

		let name = match option
		{
			Some(Token {
				kind: Identifier(name),
				span,
			}) =>
			{
				end = span.end;
				name
			}
			_ => bail!(source.error(Span { start, end }, error::FUNCTION_NAME)),
		};

		let span_left = match stream.next()
		{
			Some(token) if matches!(token.kind, ParenthesisLeft) => token.span,
			_ =>
			{
				bail!(source.error(Span { start, end }, error::EXPECT_PARENTHESIS))
			}
		};

		let parameters = utils::params(stream, source)?;

		match stream.next()
		{
			Some(token) if matches!(token.kind, ParenthesisRight) => token.span,
			_ => bail!(source.error(span_left, error::PARENTHESIS)),
		};

		let type_string = match stream.peek()
		{
			Some(Token {
				kind: ArrowRight, ..
			}) =>
			{
				let span = stream.next().expect("ArrowRight Token").span;
				match stream.next()
				{
					Some(Token {
						kind: Identifier(name),
						span,
					}) =>
					{
						end = span.end;
						Some(name)
					}
					_ => bail!(source.error(span, error::EXPECTED_RETURN_TYPE)),
				}
			}
			_ => None,
		};

		let return_type = match type_string.as_deref()
		{
			Some("Number") => Type::Number,
			Some("String") => Type::String,
			Some("Bool") => Type::Boolean,
			Some("Unit") => Type::Unit,
			None => Type::Unit,
			_ => unreachable!(),
		};

		match stream.peek()
		{
			Some(Token {
				kind: BraceLeft, ..
			}) =>
			{
				let body = utils::block(stream, source)?;
				let span = Span {
					start,
					end: body.span.end,
				};
				Ok(Self {
					span,
					name,
					return_type,
					parameters,
					body,
				})
			}

			_ => bail!(source.error(Span { start, end }, error::BLOCK_AFTER)),
		}
	}
}
