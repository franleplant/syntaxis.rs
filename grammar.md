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

P                          |  First +
----------------------------------------------------
Re -> Lit Ops              | Lit
Re -> ( Re ) Ops           | (
                           |
Ops -> * ReL               | *
Ops -> + ReL               | +
Ops -> | Re                | |
Ops -> Re                  | Lit, (
Ops -> Lambda              | eof, ), Lambda
                           |
ReL -> Re                  | Lit, (
ReL -> Lambda              | eof, ), Lambda
