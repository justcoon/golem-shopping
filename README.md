# golem-shopping


* cart - dependencies: pricing, product, order
* order - dependencies: pricing, product

```
golem-cli stubgen initialize-workspace --targets order  --targets product --targets pricing --callers cart --callers order
```