/*
 * leomeinel_sample is a cargo crate.
 * Copyright © 2022 Leopold Meinel & contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see https://github.com/LeoMeinel/leomeinel_sample/blob/main/LICENSE
 */

//! # leomeinel_sample
//!
//! `leomeinel_sample` is a collection of utilities to perform checks on numbers

pub use self::utilities::is_dividable_by_two;
pub use self::utilities::is_three;

pub mod utilities {
    /// Returns true if input is equal to 3
    ///
    /// # Examples
    ///
    /// ```
    /// let arg = 3;
    /// let arg_is_three = leomeinel_sample::is_three(arg);
    /// assert_eq!(true, arg_is_three);
    /// ```
    pub fn is_three(x: i128) -> bool {
        x == 3
    }

    /// Returns true if input is dividable by 2
    ///
    /// # Examples
    ///
    /// ```
    /// let arg = 4;
    /// let arg_is_dividable_by_two = leomeinel_sample::is_dividable_by_two(arg);
    /// assert_eq!(true, arg_is_dividable_by_two);
    /// ```
    pub fn is_dividable_by_two(x: i128) -> bool {
        x % 2 == 0
    }
}
