$$N_{2} = -N_{1}$$
$$A_{1} = \arccos\left(\frac{V_{1} \cdot N_{1}}{ \left| V_{1} \right| \cdot \left| N_{1} \right| }\right)$$
$$A_{2} = \arcsin\left(\frac{n_{1}\cdot\sin\left(A_{1}\right)}{n_{2}}\right)$$
---
$$U=
\begin{bmatrix}
    a V_{1}.x + b N_{1}.x \\
    a V_{1}.y + b N_{1}.y
\end{bmatrix}
$$
---
$$U \cdot N_{1} = 0$$
---
$$U.x \cdot N_{1}.x + U.y \cdot N_{1}.y=0$$
---
$$N_{1}.x\cdot\left(aV_{1}.x+bN_{1}.x\right)+N_{1}.y\cdot\left(aV_{1}.y+bN_{1}.y\right)=0$$
$$b \left( N_{1}.x^{2} + N_{1}.y^{2} \right) + aV_{1}.x N_{1}.x + aV_{1}.y N_{1}.y=0$$
$$b = \frac{-a V_{1}.x N_{1}.x - a V_{1}.y N_{1}.y}{N_{1}.x^{2} + N_{1}.y^{2}}$$
$$b = \frac{-a V_{1} N_{1}}{N_{1}^{2}}$$
---
$$\sqrt{\left( a V_{1}.x + b N_{1}.x \right)^{2} + \left( a V_{1}.y + b N_{1}.y \right)^{2}} = \pm 1$$
$$\left( a V_{1}.x + b N_{1}.x \right)^{2} + \left( a V_{1}.y + b N_{1}.y \right)^{2} = \pm 1$$
$$a^{2} V_{1}.x^{2} + 2 a b V_{1}.x N_{1}.x + b^{2} N_{1}.x^{2} + a^{2} V_{1}.y^{2} + 2 a b V_{1}.y N_{1}.y + b^{2} N_{1}.y^{2} = \pm 1$$
$$a^{2} \left( V_{1}.x^{2} + V_{1}.y^{2} \right) + 2 a b \left( V_{1}.x N_{1}.x + V_{1}.y N_{1}.y \right) + b^{2} \left( N_{1}.x^{2} + N_{1}.y^{2} \right) = \pm 1$$
$$a^{2} V_{1}^{2} + 2 a b V_{1} N_{1} + b^{2} N_{1}^{2} = \pm 1$$
---
Set $b = \frac{-a V_{1} N_{1}}{N_{1}^{2}}$
$$a^{2} V_{1}^{2} + 2 a \left( \frac{-a V_{1} N_{1}}{N_{1}^{2}} \right) V_{1} N_{1} + \left( \frac{-a V_{1} N_{1}}{N_{1}^{2}} \right)^{2} N_{1}^{2} = \pm 1$$
$$a^{2} V_{1}^{2} - 2 \left( \frac{ \left( a V_{1} N_{1} \right)^{2}}{N_{1}^{2}} \right) + \left( \frac{ \left( a V_{1} N_{1} \right)^{2} }{N_{1}^{2}} \right) = \pm 1$$
$$a^{2} V_{1}^{2} - \left( \frac{ \left( a V_{1} N_{1} \right)^{2}}{N_{1}^{2}} \right) = \pm 1$$
$$a^{2} V_{1}^{2} - \left( \frac{ a^{2} \left( V_{1} N_{1} \right)^{2}}{N_{1}^{2}} \right) = \pm 1$$
$$a^{2} \left( V_{1}^{2} - \frac{ \left( V_{1} N_{1} \right)^{2}}{N_{1}^{2}} \right) = \pm 1$$
$$a = - \sqrt{\frac{1}{\left( V_{1}^{2} - \frac{ \left( V_{1} N_{1} \right)^{2}}{N_{1}^{2}} \right)}}$$
---
$$U = a V_{1} + b N_{1} = 
\begin{bmatrix}
    a V_{1}.x + b N_{1}.x \\
    a V_{1}.y + b N_{1}.y
\end{bmatrix}
$$
---
$$V_{2}= -\left( N_{2}\cos\left(A_{2}\right) + U\sin\left(A_{2} \right) \right)$$
---
TIR happens if
$$\frac{n_{1}\sin\left(A_{1}\right)}{n_{2}}>1$$
$$n_{1}\sin\left(A_{1}\right) > n_{2}$$
$$\sin\left(A_{1}\right) > \frac{n_{2}}{n_{1}}$$
