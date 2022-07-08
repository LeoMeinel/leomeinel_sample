#!/bin/sh
#
# leomeinel_sample is a cargo crate.
# Copyright © 2022 Leopold Meinel & contributors
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program. If not, see https://github.com/LeoMeinel/leomeinel_sample/blob/main/LICENSE
#

debug_project=$1
gpg -b ./target/debug/"$debug_project"
gpg --verify ./target/debug/"$debug_project".sig ./target/debug/"$debug_project"
release_project=$1
gpg -b ./target/release/"$release_project"
gpg --verify ./target/release/"$release_project".sig ./target/release/"$release_project"
