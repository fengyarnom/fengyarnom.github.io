---
title: 定积分 I = \int_{0}^{\pi} \frac{x\left | \sin x\cos x \right | }{1+\cos x^2} dx
date: 2023-10-12 12:40:07
tags:
- 微积分
categories:
- 微积分
published: false
---

$$
I = \int_{0}^{\pi} \frac{x\left | \sin x\cos x \right | }{1+\cos x^2} dx
$$

这道题用到了区间再现，即:
$$
\begin{align*}
  I &= \int_{a}^{b}f(x)dx \underset{dx = -dt}{\overset{x = a+b-t}{ =\\!=\\!=\\!=\\!=}} \int_{b}^{a}f(a+b-t)\cdot (-dt) \\\\
  &= \int_{a}^{b} f(a+b-x)dx\\
\end{align*}
$$
通过上面的原理，我们可以得到下面的这样一个公式：
$$
\int_{0}^{\pi}xf(sinx)dx = \frac{\pi}{2}\int_{0}^{\pi}f(sinx)dx=\pi\int_{0}^{\frac{\pi}{2}}f(sinx)dx
$$
$$

