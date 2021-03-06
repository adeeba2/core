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

// Base Bits= 28
//  nist521 Curve Modulus
static let Modulus:[Chunk] = [0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0x1FFFF]
static let ROI:[Chunk] = [0xFFFFFFE,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0x1FFFF]
static let R2modp:[Chunk] = [0x400000,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]
static let MConst:Chunk = 0x1

//  nist521 Curve
static let CURVE_Cof_I:Int = 1
static let CURVE_Cof:[Chunk] = [0x1,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]
static let CURVE_A:Int = -3
static let CURVE_B_I:Int = 0
static let CURVE_B:[Chunk] = [0xB503F00,0x451FD46,0xC34F1EF,0xDF883D2,0xF073573,0xBD3BB1B,0xB1652C0,0xEC7E937,0x6193951,0xF109E15,0x489918E,0x15F3B8B,0x25B99B3,0xEEA2DA7,0xB68540,0x929A21A,0xE1C9A1F,0x3EB9618,0x5195]
static public let CURVE_Order:[Chunk] = [0x1386409,0x6FB71E9,0xC47AEBB,0xC9B8899,0x5D03BB5,0x48F709A,0xB7FCC01,0xBF2F966,0x1868783,0xFFFFFA5,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0xFFFFFFF,0x1FFFF]
static public let CURVE_Gx:[Chunk] = [0x2E5BD66,0x7E7E31C,0xA429BF9,0xB3C1856,0x8DE3348,0x27A2FFA,0x8FE1DC1,0xEFE7592,0x14B5E77,0x4D3DBAA,0x8AF606B,0xB521F82,0x139053F,0x429C648,0x62395B4,0x9E3ECB6,0x404E9CD,0x8E06B70,0xC685]
static public let CURVE_Gy:[Chunk] = [0xFD16650,0xBE94769,0x2C24088,0x7086A27,0x761353C,0x13FAD0,0xC550B9,0x5EF4264,0x7EE7299,0x3E662C9,0xFBD1727,0x446817A,0x449579B,0xD998F54,0x42C7D1B,0x5C8A5FB,0xA3BC004,0x296A789,0x11839]


#endif

#if D64

// Base Bits= 60
//  nist521 Curve Modulus
static let Modulus:[Chunk] = [0xFFFFFFFFFFFFFFF,0xFFFFFFFFFFFFFFF,0xFFFFFFFFFFFFFFF,0xFFFFFFFFFFFFFFF,0xFFFFFFFFFFFFFFF,0xFFFFFFFFFFFFFFF,0xFFFFFFFFFFFFFFF,0xFFFFFFFFFFFFFFF,0x1FFFFFFFFFF]
static let ROI:[Chunk] = [0xFFFFFFFFFFFFFFE,0xFFFFFFFFFFFFFFF,0xFFFFFFFFFFFFFFF,0xFFFFFFFFFFFFFFF,0xFFFFFFFFFFFFFFF,0xFFFFFFFFFFFFFFF,0xFFFFFFFFFFFFFFF,0xFFFFFFFFFFFFFFF,0x1FFFFFFFFFF]
static let R2modp:[Chunk] = [0x4000000000,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]
static let MConst:Chunk = 0x1

//  nist521 Curve
static let CURVE_Cof_I:Int = 1
static let CURVE_Cof:[Chunk] = [0x1,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]
static let CURVE_A:Int = -3
static let CURVE_B_I:Int = 0
static let CURVE_B:[Chunk] = [0xF451FD46B503F00,0x73DF883D2C34F1E,0x2C0BD3BB1BF0735,0x3951EC7E937B165,0x9918EF109E15619,0x5B99B315F3B8B48,0xB68540EEA2DA72,0x8E1C9A1F929A21A,0x51953EB961]
static public let CURVE_Order:[Chunk] = [0xB6FB71E91386409,0xB5C9B8899C47AEB,0xC0148F709A5D03B,0x8783BF2F966B7FC,0xFFFFFFFFFFA5186,0xFFFFFFFFFFFFFFF,0xFFFFFFFFFFFFFFF,0xFFFFFFFFFFFFFFF,0x1FFFFFFFFFF]
static public let CURVE_Gx:[Chunk] = [0x97E7E31C2E5BD66,0x48B3C1856A429BF,0xDC127A2FFA8DE33,0x5E77EFE75928FE1,0xF606B4D3DBAA14B,0x39053FB521F828A,0x62395B4429C6481,0x404E9CD9E3ECB6,0xC6858E06B7]
static public let CURVE_Gy:[Chunk] = [0x8BE94769FD16650,0x3C7086A272C2408,0xB9013FAD076135,0x72995EF42640C55,0xD17273E662C97EE,0x49579B446817AFB,0x42C7D1BD998F544,0x9A3BC0045C8A5FB,0x11839296A78]

#endif

}

