# Grammar for the regexp meta lang


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

V     |  First             |  Follow
--------------------------------------------
Re    |  Lit, (            |  eof, )
Ops   |  *, +, |, Lambda   |  eof, )
ReL   |  Lit, (            |  eof, )
Lit   |                    |
(     |                    |
)     |                    |
*     |                    |
+     |                    |
|     |                    |


