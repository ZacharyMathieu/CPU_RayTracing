$$R = R_{0} + r R_{1}$$
$$C \implies (x - C_x)^2 + (y - C_y)^2 + (z - C_z)^2 = C_r^2$$
---
$$(r R_{1}.x + R_{0}.x - C_x)^2 + (r R_{1}.y + R_{0}.y - C_y)^2 + (r R_{1z} + R_{0z} - C_z)^2 = C_r^2$$
---
Let's try solving for $x$ only:
$$(r R_{1}.x + R_{0}.x - C_x)^2 = C_r^2$$
$$\left( r^{2} R_{1}.x^{2} + r R_{1}.x R_{0}.x - r R_{1}.x C.x \right) + \left( r R_{1}.x R_{0}.x + R_{0}.x^{2} - R_{0}.x C.x \right) + \left( -r R_{1}.x C.x - R_{0}.x C.x + C.x^{2} \right) - C_{R}^{2} = 0$$
$$r^{2} \left[ R_{1}.x^{2} \right] + r \left[ 2 R_{1}.x R_{0}.x - 2 R_{1}.x C.x \right] + \left[ R_{0}.x^{2} - 2 R_{0}.x C.x + C.x^{2} - C_{R}^{2} \right] = 0$$
Add $y$ and $z$ back in and solve the quadratic:
$$a = R_{1}.x^{2} + R_{1}.y^{2} + R_{1}.z^{2}$$
$$b = 2 R_{1}.x R_{0}.x - 2 R_{1}.x C.x + 2 R_{1}.y R_{0}.y - 2 R_{1}.y C.y + 2 R_{1}.z R_{0}.z - 2 R_{1}.z C.z$$
$$c = R_{0}.x^{2} - 2 R_{0}.x C.x + C.x^{2} + R_{0}.y^{2} - 2 R_{0}.y C.y + C.y^{2} + R_{0}.z^{2} - 2 R_{0}.z C.z + C.z^{2} - C_{R}^{2}$$
$$\Delta = b^2 - 4 a c$$
If $\Delta \geq 0$, then
$$r = \frac{-b \pm \sqrt{\Delta}}{2a}$$
