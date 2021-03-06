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
use crate::nums512e::big::NLEN;

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

// nums512e Curve
pub const CURVE_COF_I: isize = 4;
pub const CURVE_A: isize = 1;
pub const CURVE_B_I: isize = -78296;
pub const CURVE_COF: [Chunk; NLEN] = [0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_B: [Chunk; NLEN] = [
    0xFFFFFFFFFFECBEF,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFF,
];
pub const CURVE_ORDER: [Chunk; NLEN] = [
    0x7468CF51BEED46D,
    0x4605786DEFECFF6,
    0xFD8C970B686F52A,
    0x636D2FCF91BA9E3,
    0xFFFFFFFFFFFB4F0,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFF,
    0x3FFFFFFF,
];
pub const CURVE_GX: [Chunk; NLEN] = [
    0x5B9AB2999EC57FE,
    0xE525427CC4F015C,
    0xDC992568904AD0F,
    0xC14EEE46730F78B,
    0xEBE273B81474621,
    0x9F4DC4A38227A17,
    0x888D3C5332FD1E7,
    0x128DB69C7A18CB7,
    0xDF8E316D,
];
pub const CURVE_GY: [Chunk; NLEN] = [
    0x26DDEC0C1E2F5E1,
    0x66D38A9BF1D01F3,
    0xA06862AECC1FD02,
    0x53F2E9963562601,
    0xB95909E834120CA,
    0x26D8259D22A92B6,
    0x7A82A256EE476F7,
    0x9D49CA7198B0F57,
    0x6D09BFF3,
];
