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

/* Note that the original curve has been transformed to an isomorphic curve with A=-3 */

use crate::arch::Chunk;
use crate::brainpool::big::NLEN;

// Base Bits= 56
// brainpool Modulus
pub const MODULUS: [Chunk; NLEN] = [
    0x13481D1F6E5377,
    0xF623D526202820,
    0x909D838D726E3B,
    0xA1EEA9BC3E660A,
    0xA9FB57DB,
];
pub const ROI: [Chunk; NLEN] = [
    0x13481D1F6E5376,
    0xF623D526202820,
    0x909D838D726E3B,
    0xA1EEA9BC3E660A,
    0xA9FB57DB,
];
pub const R2MODP: [Chunk; NLEN] = [
    0x9E04F49B9A3787,
    0x29317218F3CF49,
    0x54E8C3CF1DBC89,
    0xBB411A3F7559CA,
    0x9773E15F,
];
pub const MCONST: Chunk = 0xA75590CEFD89B9;

// brainpool Curve
pub const CURVE_COF_I: isize = 1;
pub const CURVE_A: isize = -3;
pub const CURVE_B_I: isize = 0;
pub const CURVE_COF: [Chunk; NLEN] = [0x1, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_B: [Chunk; NLEN] = [
    0xE58101FEE92B04,
    0xEBC4AF2F49256A,
    0x733D0B76B7BF93,
    0x30D84EA4FE66A7,
    0x662C61C4,
];
pub const CURVE_ORDER: [Chunk; NLEN] = [
    0x1E0E82974856A7,
    0x7AA3B561A6F790,
    0x909D838D718C39,
    0xA1EEA9BC3E660A,
    0xA9FB57DB,
];
pub const CURVE_GX: [Chunk; NLEN] = [
    0xA191562E1305F4,
    0x42C47AAFBC2B79,
    0xB23A656149AFA1,
    0xC1CFE7B7732213,
    0xA3E8EB3C,
];
pub const CURVE_GY: [Chunk; NLEN] = [
    0xABE8F35B25C9BE,
    0xB6DE39D027001D,
    0xE14644417E69BC,
    0x3439C56D7F7B22,
    0x2D996C82,
];
