/*
   Copyright (C) 2019 MIRACL UK Ltd.

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as
    published by the Free Software Foundation, either version 3 of the
    License, or (at your option) any later version.


    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

     https://www.gnu.org/licenses/agpl-3.0.en.html

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.

   You can be released from the requirements of the license by purchasing     
   a commercial license. Buying such a license is mandatory as soon as you
   develop commercial activities involving the MIRACL Core Crypto SDK
   without disclosing the source code of your own applications, or shipping
   the MIRACL Core Crypto SDK with a closed source product.     
*/

use crate::arch::Chunk;
use crate::nums256e::big::NLEN;

// Base Bits= 29
// nums256 Modulus
pub const MODULUS: [Chunk; NLEN] = [
    0x1FFFFF43, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF,
    0xFFFFFF,
];
pub const ROI: [Chunk; NLEN] = [
    0x1FFFFF42, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF,
    0xFFFFFF,
];
pub const R2MODP: [Chunk; NLEN] = [0x22E2400, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
pub const MCONST: Chunk = 0xBD;

// nums256e Curve
pub const CURVE_COF_I: isize = 4;
pub const CURVE_A: isize = 1;
pub const CURVE_B_I: isize = -15342;
pub const CURVE_COF: [Chunk; NLEN] = [0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_B: [Chunk; NLEN] = [
    0x1FFFC355, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF,
    0xFFFFFF,
];
pub const CURVE_ORDER: [Chunk; NLEN] = [
    0xEDD4AF5, 0x123D8C87, 0x1650E6C6, 0xAB54A5E, 0x419, 0x0, 0x0, 0x0, 0x400000,
];
pub const CURVE_GX: [Chunk; NLEN] = [
    0xEED13DA, 0x6F60481, 0x20D61A8, 0x13141DC6, 0x9BD60C3, 0x1EAFB490, 0xDF73478, 0x1F6D5D44,
    0x8A7514,
];
pub const CURVE_GY: [Chunk; NLEN] = [
    0x198A89E6, 0x1D30B73B, 0x15BB4CB, 0x1EC3B021, 0x18010715, 0x12ECD325, 0x171F3A59, 0x13FB3B24,
    0x44D53E,
];
