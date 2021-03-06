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
//
//  rom.swift
//
//  Created by Michael Scott on 12/06/2015.
//  Copyright (c) 2015 Michael Scott. All rights reserved.
//

public struct ROM{
 
#if D32

//  c13318 Curve Modulus
// Base Bits= 29
static let Modulus:[Chunk] = [0x1FFFFFED,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x7FFFFF]
static let R2modp:[Chunk] = [0x169000,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]
static let ROI:[Chunk] = [0xA0EA0B0,0x770D93A,0xBF91E31,0x6300D5A,0x1D7A72F4,0x4C9EFD,0x1C2CAD34,0x1009F83B,0x2B8324]
static let MConst:Chunk = 0x13

//*** rom curve parameters *****
// Base Bits= 29

static let CURVE_A:Int = -3
static let CURVE_Cof_I:Int = 1
static public let CURVE_Cof:[Chunk] = [0x1,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]
static let CURVE_B_I:Int = 13318
static public let CURVE_B:[Chunk] = [0x3406,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]
static public let CURVE_Order:[Chunk] = [0xDC2CBE3,0x1BE896E2,0x1AE345BA,0xCA9F07B,0xF4F,0x0,0x0,0x0,0x800000]
static public let CURVE_Gx:[Chunk] = [0x5,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]
static public let CURVE_Gy:[Chunk] = [0xB6EAD0B,0x6469AA3,0x5B6D6E,0x1996099E,0x166369D4,0x18728B34,0x1BC4E058,0x1B24D794,0x6675AA]

#endif

#if D64

//  c13318 Curve Modulus
// Base Bits= 56
static let Modulus:[Chunk] = [0xFFFFFFFFFFFFED,0xFFFFFFFFFFFFFF,0xFFFFFFFFFFFFFF,0xFFFFFFFFFFFFFF,0x7FFFFFFF]
static let R2modp:[Chunk] = [0xA4000000000000,0x5,0x0,0x0,0x0]
static let ROI:[Chunk] = [0xEE1B274A0EA0B0,0x1806AD2FE478C4,0x993DFBD7A72F43,0x4FC1DF0B2B4D00,0x2B832480]
static let MConst:Chunk = 0x13

//*** rom curve parameters *****
// Base Bits= 56

static let CURVE_A:Int = -3
static let CURVE_Cof_I:Int = 1
static public let CURVE_Cof:[Chunk] = [0x1,0x0,0x0,0x0,0x0]
static let CURVE_B_I:Int = 13318
static public let CURVE_B:[Chunk] = [0x3406,0x0,0x0,0x0,0x0]
static public let CURVE_Order:[Chunk] = [0x7D12DC4DC2CBE3,0x54F83DEB8D16EB,0xF4F6,0x0,0x80000000]
static public let CURVE_Gx:[Chunk] = [0x5,0x0,0x0,0x0,0x0]
static public let CURVE_Gy:[Chunk] = [0xC8D3546B6EAD0B,0xCB04CF016DB5B8,0xE5166966369D4C,0x26BCA6F1381630,0x6675AAD9]

#endif

}

