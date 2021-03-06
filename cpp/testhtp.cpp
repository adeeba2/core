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

/* test driver and function exerciser for HTP API Functions */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "core.h"
#include "randapi.h"
#include "ecp_NIST256.h"

#define RO

using namespace core;
using namespace NIST256;
using namespace NIST256_BIG;
using namespace NIST256_FP;

#define CEIL(a,b) (((a)-1)/(b)+1)

/* WORK IN PROGRESS - PLEASE IGNORE */


/* https://datatracker.ietf.org/doc/draft-irtf-cfrg-hash-to-curve/ */
static void hash_to_field(int hash,int hlen,FP *u,octet *DST,octet *M, int ctr)
{
    int i,j,L,k;
    BIG q,w;
    DBIG dx;
    char okm[512],fd[256];
    octet OKM = {0,sizeof(okm),okm};

    BIG_rcopy(q, Modulus);
    k=BIG_nbits(q);
    L=CEIL(k+CEIL(k,2),8);
    XMD_Expand(hash,hlen,&OKM,L*ctr,DST,M);
    for (i=0;i<ctr;i++)
    {
        for (j=0;j<L;j++)
            fd[j]=OKM.val[i*L+j];
        
        BIG_dfromBytesLen(dx,fd,L);
        BIG_dmod(w,dx,q);
        FP_nres(&u[i],w);
    }
}

int htp_NIST256(char *mess)
{
    int res=0;
    FP u[2];
    ECP P,P1;
    char dst[50];
    char msg[2000];
    octet MSG = {0,sizeof(msg),msg};
    octet DST = {0,sizeof(dst),dst};

    OCT_jstring(&MSG,mess);

#ifdef RO
    OCT_jstring(&DST,(char *)"P256_XMD:SHA-256_SSWU_RO_TESTGEN");
    hash_to_field(MC_SHA2,HASH_TYPE_NIST256,u,&DST,&MSG,2);
    printf("u[0]= "); FP_output(&u[0]); printf("\n");
    printf("u[1]= "); FP_output(&u[1]); printf("\n");
    ECP_map2point(&P,&u[0]);
    printf("Q[0]= "); ECP_output(&P);
    ECP_map2point(&P1,&u[1]);
    printf("Q[1]= "); ECP_output(&P1);
    ECP_add(&P,&P1);
#else
    OCT_jstring(&DST,(char *)"P256_XMD:SHA-256_SSWU_NU_TESTGEN");
    hash_to_field(MC_SHA2,HASH_TYPE_NIST256,u,&DST,&MSG,1);
    printf("u[0]= "); FP_output(&u[0]); printf("\n");
    ECP_map2point(&P,&u[0]);
    printf("Q= "); ECP_output(&P);
#endif
    ECP_cfp(&P);
    ECP_affine(&P);
    printf("+P= "); ECP_output(&P); printf("\n");
    return res;
}

int main()
{
    int i, res;
    unsigned long ran;

    char raw[100];
    octet RAW = {0, sizeof(raw), raw};
    csprng RNG;                // Crypto Strong RNG

    time((time_t *)&ran);

    RAW.len = 100;              // fake random seed source
    RAW.val[0] = ran;
    RAW.val[1] = ran >> 8;
    RAW.val[2] = ran >> 16;
    RAW.val[3] = ran >> 24;
    for (i = 4; i < 100; i++) RAW.val[i] = i;

    CREATE_CSPRNG(&RNG, &RAW);  // initialise strong RNG

    printf("%d bit build", CHUNK);

    printf("\nTesting HTP for curve NIST256\n");
    htp_NIST256((char *)"");
    htp_NIST256((char *)"abc");
    htp_NIST256((char *)"abcdef0123456789");
    htp_NIST256((char *)"a512_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");

    KILL_CSPRNG(&RNG);
}


