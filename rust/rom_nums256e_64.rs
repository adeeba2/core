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

// Base Bits= 56
// nums256 Modulus
pub const MODULUS: [Chunk; NLEN] = [
    0xFFFFFFFFFFFF43,
    0xFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFF,
    0xFFFFFFFF,
];
pub const ROI: [Chunk; NLEN] = [
    0xFFFFFFFFFFFF42,
    0xFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFF,
    0xFFFFFFFF,
];
pub const R2MODP: [Chunk; NLEN] = [0x89000000000000, 0x8B, 0x0, 0x0, 0x0];
pub const MCONST: Chunk = 0xBD;

// nums256e Curve
pub const CURVE_COF_I: isize = 4;
pub const CURVE_A: isize = 1;
pub const CURVE_B_I: isize = -15342;
pub const CURVE_COF: [Chunk; NLEN] = [0x4, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_B: [Chunk; NLEN] = [
    0xFFFFFFFFFFC355,
    0xFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFF,
    0xFFFFFFFF,
];
pub const CURVE_ORDER: [Chunk; NLEN] =
    [0x47B190EEDD4AF5, 0x5AA52F59439B1A, 0x4195, 0x0, 0x40000000];
pub const CURVE_GX: [Chunk; NLEN] = [
    0xDEC0902EED13DA,
    0x8A0EE3083586A0,
    0x5F69209BD60C39,
    0x6AEA237DCD1E3D,
    0x8A7514FB,
];
pub const CURVE_GY: [Chunk; NLEN] = [
    0xA616E7798A89E6,
    0x61D810856ED32F,
    0xD9A64B8010715F,
    0xD9D925C7CE9665,
    0x44D53E9F,
];
