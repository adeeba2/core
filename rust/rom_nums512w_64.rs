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
use crate::nums512w::big::NLEN;

// Base Bits= 60
// nums512 Modulus
pub const MODULUS: [Chunk; NLEN] = [
    0xFFFFFFFFFFFFDC7,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFF,
];
pub const ROI: [Chunk; NLEN] = [
    0xFFFFFFFFFFFFDC6,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFF,
];
pub const R2MODP: [Chunk; NLEN] = [0x100000000000000, 0x4F0B, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
pub const MCONST: Chunk = 0x239;

// nums512w Curve
pub const CURVE_COF_I: isize = 1;
pub const CURVE_A: isize = -3;
pub const CURVE_B_I: isize = 121243;
pub const CURVE_COF: [Chunk; NLEN] = [0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_B: [Chunk; NLEN] = [0x1D99B, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_ORDER: [Chunk; NLEN] = [
    0xE153F390433555D,
    0x568B36607CD243C,
    0x258ED97D0BDC63B,
    0xA4FB94E7831B4FC,
    0xFFFFFFFFFFF5B3C,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFF,
];
pub const CURVE_GX: [Chunk; NLEN] = [
    0xC8287958CABAE57,
    0x5D60137D6F5DE2D,
    0x94286255615831D,
    0xA151076B359E937,
    0xC25306D9F95021,
    0x3BB501F6854506E,
    0x2A03D3B5298CAD8,
    0x141D0A93DA2B700,
    0x3AC03447,
];
pub const CURVE_GY: [Chunk; NLEN] = [
    0x3A08760383527A6,
    0x2B5C1E4CFD0FE92,
    0x1A840B25A5602CF,
    0x15DA8B0EEDE9C12,
    0x60C7BD14F14A284,
    0xDEABBCBB8C8F4B2,
    0xC63EBB1004B97DB,
    0x29AD56B3CE0EEED,
    0x943A54CA,
];
