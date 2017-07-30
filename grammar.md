# Grammar for the regexp meta lang

### Grammar

```
Re -> Lit Ops
Re -> ( Re ) Ops

Ops -> * ReL
Ops -> + ReL
Ops -> | Re
Ops -> Re
Ops -> Lambda

ReL -> Re
ReL -> Lambda

```
### First and Follow calc

V     |  First             |  Follow
----------------------------------------
Re    |  Lit, (            |  eof, )
Ops   |  *, +, |, Lambda   |  eof, )
ReL   |  Lit, (, Lambda    |  eof, )
Lit   |                    |
(     |                    |
)     |                    |
*     |                    |
+     |                    |
|     |                    |



### First+ for each prod

N  |  P                          |  First +
---|------------------------------------------------------
0  |  Re -> Lit Ops              | Lit
1  |  Re -> ( Re ) Ops           | (
   |                             |
2  |  Ops -> * ReL               | *
3  |  Ops -> + ReL               | +
4  |  Ops -> | Re                | |
5  |  Ops -> Re                  | Lit, (
6  |  Ops -> Lambda              | eof, ), Lambda
   |                             |
7  |  ReL -> Re                  | Lit, (
8  |  ReL -> Lambda              | eof, ), Lambda
