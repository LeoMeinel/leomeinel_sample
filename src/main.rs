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

#[allow(clippy::bool_assert_comparison)]
fn main() {
    let arg = 3;
    let arg_is_three = leomeinel_sample::is_three(arg);
    assert_eq!(true, arg_is_three);
    let arg = 4;
    let arg_is_dividable_by_two = leomeinel_sample::is_dividable_by_two(arg);
    assert_eq!(true, arg_is_dividable_by_two);
}
