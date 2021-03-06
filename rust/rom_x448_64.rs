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
use crate::x448::big::NLEN;

// Base Bits= 58
// Goldilocks modulus
pub const MODULUS: [Chunk; NLEN] = [
    0x3FFFFFFFFFFFFFF,
    0x3FFFFFFFFFFFFFF,
    0x3FFFFFFFFFFFFFF,
    0x3FBFFFFFFFFFFFF,
    0x3FFFFFFFFFFFFFF,
    0x3FFFFFFFFFFFFFF,
    0x3FFFFFFFFFFFFFF,
    0x3FFFFFFFFFF,
];
pub const ROI: [Chunk; NLEN] = [
    0x3FFFFFFFFFFFFFE,
    0x3FFFFFFFFFFFFFF,
    0x3FFFFFFFFFFFFFF,
    0x3FBFFFFFFFFFFFF,
    0x3FFFFFFFFFFFFFF,
    0x3FFFFFFFFFFFFFF,
    0x3FFFFFFFFFFFFFF,
    0x3FFFFFFFFFF,
];
pub const R2MODP: [Chunk; NLEN] = [0x200000000, 0x0, 0x0, 0x0, 0x3000000, 0x0, 0x0, 0x0];
pub const MCONST: Chunk = 0x1;

// x448 curve
pub const CURVE_A:isize = 156326;
pub const CURVE_COF_I:isize = 4;
pub const CURVE_COF:[Chunk;NLEN]=[0x4,0x0,0x0,0x0,0x0,0x0,0x0,0x0];
pub const CURVE_B_I:isize = 0;
pub const CURVE_B:[Chunk;NLEN]=[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0];
pub const CURVE_ORDER:[Chunk;NLEN]=[0x378C292AB5844F3,0x3309CA37163D548,0x1B49AED63690216,0x3FDF3288FA7113B,0x3FFFFFFFFFFFFFF,0x3FFFFFFFFFFFFFF,0x3FFFFFFFFFFFFFF,0xFFFFFFFFFF];
pub const CURVE_GX:[Chunk;NLEN]=[0x5,0x0,0x0,0x0,0x0,0x0,0x0,0x0];
pub const CURVE_GY:[Chunk;NLEN]=[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0];
