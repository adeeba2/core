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

/* Fixed Data in ROM - Field and Curve parameters */


package org.miracl.core.BN254CX;

public class ROM
{
// Base Bits= 56
	public static final long[] Modulus= {0x6623EF5C1B55B3L,0xD6EE18093EE1BEL,0x647A6366D3243FL,0x8702A0DB0BDDFL,0x24000000L};
	public static final long[] ROI= {0x6623EF5C1B55B2L,0xD6EE18093EE1BEL,0x647A6366D3243FL,0x8702A0DB0BDDFL,0x24000000L};
	public static final long[] R2modp= {0x466A0618A0800AL,0x2B3A22543056A3L,0x148515B09C6600L,0xEC9EA5606BDF50L,0x1C992E66L};
	public static final long MConst= 0x4E205BF9789E85L;

	public static final int CURVE_A= 0;
	public static final int CURVE_B_I= 2;
	public static final int CURVE_Cof_I= 1;
	public static final long[] CURVE_B= {0x2L,0x0L,0x0L,0x0L,0x0L};
	public static final long[] CURVE_Order= {0x11C0A636EB1F6DL,0xD6EE0CC906CEBEL,0x647A6366D2C43FL,0x8702A0DB0BDDFL,0x24000000L};
	public static final long[] CURVE_Gx= {0x6623EF5C1B55B2L,0xD6EE18093EE1BEL,0x647A6366D3243FL,0x8702A0DB0BDDFL,0x24000000L};
	public static final long[] CURVE_Gy= {0x1L,0x0L,0x0L,0x0L,0x0L};

	public static final long[] CURVE_Bnx= {0x3C012B1L,0x40L,0x0L,0x0L,0x0L};
	public static final long[] CURVE_Cof= {0x1L,0x0L,0x0L,0x0L,0x0L};
	public static final long[] CURVE_Cru= {0xE0931794235C97L,0xDF6471EF875631L,0xCA83F1440BDL,0x480000L,0x0L};
	public static final long[] Fra= {0xD9083355C80EA3L,0x7326F173F8215BL,0x8AACA718986867L,0xA63A0164AFE18BL,0x1359082FL};
	public static final long[] Frb= {0x8D1BBC06534710L,0x63C7269546C062L,0xD9CDBC4E3ABBD8L,0x623628A900DC53L,0x10A6F7D0L};

	public static final long[] CURVE_Pxa= {0x851CEEE4D2EC74L,0x85BFA03E2726C0L,0xF5C34BBB907CL,0x7053B256358B25L,0x19682D2CL};
	public static final long[] CURVE_Pxb= {0xA58E8B2E29CFE1L,0x97B0C209C30F47L,0x37A8E99743F81BL,0x3E19F64AA011C9L,0x1466B9ECL};
	public static final long[] CURVE_Pya= {0xFBFCEBCF0BE09FL,0xB33D847EC1B30CL,0x157DAEE2096361L,0x72332B8DD81E22L,0xA79EDD9L};
	public static final long[] CURVE_Pyb= {0x904B228898EE9DL,0x4EA569D2EDEBEDL,0x512D8D3461C286L,0xECC4C09035C6E4L,0x6160C39L};


	public static final long[][] CURVE_W= {{0x546349162FEB83L,0xB40381200L,0x6000L,0x0L,0x0L},{0x7802561L,0x80L,0x0L,0x0L,0x0L}};
	public static final long[][][] CURVE_SB= {{{0x5463491DB010E4L,0xB40381280L,0x6000L,0x0L,0x0L},{0x7802561L,0x80L,0x0L,0x0L,0x0L}},{{0x7802561L,0x80L,0x0L,0x0L,0x0L},{0xBD5D5D20BB33EAL,0xD6EE0188CEBCBDL,0x647A6366D2643FL,0x8702A0DB0BDDFL,0x24000000L}}};
	public static final long[][] CURVE_WB= {{0x1C2118567A84B0L,0x3C012B040L,0x2000L,0x0L,0x0L},{0xCDF995BE220475L,0x94EDA8CA7F9A36L,0x8702A0DC07EL,0x300000L,0x0L},{0x66FCCAE0F10B93L,0x4A76D4653FCD3BL,0x4381506E03FL,0x180000L,0x0L},{0x1C21185DFAAA11L,0x3C012B0C0L,0x2000L,0x0L,0x0L}};
	public static final long[][][] CURVE_BB= {{{0x11C0A6332B0CBDL,0xD6EE0CC906CE7EL,0x647A6366D2C43FL,0x8702A0DB0BDDFL,0x24000000L},{0x11C0A6332B0CBCL,0xD6EE0CC906CE7EL,0x647A6366D2C43FL,0x8702A0DB0BDDFL,0x24000000L},{0x11C0A6332B0CBCL,0xD6EE0CC906CE7EL,0x647A6366D2C43FL,0x8702A0DB0BDDFL,0x24000000L},{0x7802562L,0x80L,0x0L,0x0L,0x0L}},{{0x7802561L,0x80L,0x0L,0x0L,0x0L},{0x11C0A6332B0CBCL,0xD6EE0CC906CE7EL,0x647A6366D2C43FL,0x8702A0DB0BDDFL,0x24000000L},{0x11C0A6332B0CBDL,0xD6EE0CC906CE7EL,0x647A6366D2C43FL,0x8702A0DB0BDDFL,0x24000000L},{0x11C0A6332B0CBCL,0xD6EE0CC906CE7EL,0x647A6366D2C43FL,0x8702A0DB0BDDFL,0x24000000L}},{{0x7802562L,0x80L,0x0L,0x0L,0x0L},{0x7802561L,0x80L,0x0L,0x0L,0x0L},{0x7802561L,0x80L,0x0L,0x0L,0x0L},{0x7802561L,0x80L,0x0L,0x0L,0x0L}},{{0x3C012B2L,0x40L,0x0L,0x0L,0x0L},{0xF004AC2L,0x100L,0x0L,0x0L,0x0L},{0x11C0A62F6AFA0AL,0xD6EE0CC906CE3EL,0x647A6366D2C43FL,0x8702A0DB0BDDFL,0x24000000L},{0x3C012B2L,0x40L,0x0L,0x0L,0x0L}}};

}

