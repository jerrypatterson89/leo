// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use leo_span::Span;

/// A node in the AST.
pub trait Node:
    std::fmt::Debug + std::fmt::Display + Clone + PartialEq + Eq + serde::Serialize + serde::de::DeserializeOwned
{
    /// Returns the span of the node.
    fn span(&self) -> Span;

    /// Sets the span of the node.
    fn set_span(&mut self, span: Span);
}

#[macro_export]
macro_rules! simple_node_impl {
    ($ty:ty) => {
        impl Node for $ty {
            fn span(&self) -> Span {
                self.span
            }

            fn set_span(&mut self, span: Span) {
                self.span = span;
            }
        }
    };
}
