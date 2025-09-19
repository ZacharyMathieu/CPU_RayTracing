$$I = R_0 + rR_1 = P_0 + p_1 P_1 + p_2 P_2$$
$$N = P_1 \times P_2$$
---
$$r = \frac{N \cdot \left( P_0 - R_0 \right)}{N \cdot R_1}$$
$$I = R_0 + \frac{N \cdot \left( P_0 - R_0 \right)}{N \cdot R_1} R_1$$
---
Now we just need to change coordinates and use $p_1$ and $p_2$ instead.
$$I = P_0 + p_1 P_1 + p_2 P_2$$
For $x$:
$$Ix = P_0x + p_1 P_1x + p_2 P_2x$$
$$p_1 P_1x = Ix - P_0x - p_2 P_2x$$
$$p_1 = \frac{Ix - P_0x - p_2 P_2x}{P_1x}$$
And for $y$:
$$Iy = P_0y + p_1 P_1y + p_2 P_2y$$
$$Iy = P_0y + \frac{Ix - P_0x - p_2 P_2x}{P_1x} P_1y + p_2 P_2y$$
$$\frac{p_2 P_2x}{P_1x} P_1y + p_2 P_2y = Iy - P_0y - \frac{Ix - P_0x}{P_1x} P_1y$$
$$p_2 \left( \frac{P_2x P_1y}{P_1x} + P_2y \right) = Iy - P_0y + \frac{P_0x P_1y - Ix P_1y}{P_1x}$$
$$p_2 \frac{P_2x P_1y + P_1x P_2y}{P_1x} = \frac{Iy P_1x - P_0y P_1x + P_0x P_1y - Ix P_1y}{P_1x}$$
$$p_2 = \frac{Iy P_1x - P_0y P_1x + P_0x P_1y - Ix P_1y}{P_1x \frac{P_2x P_1y + P_1x P_2y}{P_1x}}$$
$$p_2 = \frac{Iy P_1x - P_0y P_1x + P_0x P_1y - Ix P_1y}{P_1y P_2x + P_1x P_2y}$$

So we have:
$$p_2 = \frac{Iy P_1x - Ix P_1y + P_0x P_1y - P_0y P_1x}{P_1y P_2x + P_1x P_2y}$$
$$p_1 = \frac{Ix - P_0x - p_2 P_2x}{P_1x} = \frac{Ix - P_0x - \left( \frac{Iy P_1x - P_0y P_1x + P_0x P_1y - Ix P_1y}{P_1y P_2x + P_1x P_2y} \right) P_2x}{P_1x}$$