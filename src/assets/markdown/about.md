# Tether Quart

## System for modeling the formation of a quadrangular tether grouping of microsatellites in low-Earth orbit

### Description

The system simulates the following process:

1. In the initial state, the satellite system is connected at a small distance from each other and forms a square.
1. The satellite system rotates in the plane of the orbit with a certain angular velocity.
1. At some point, the satellites separate with some relative velocity.
1. The satellites have low-thrust engines that work for a while.
1. The tension forces of the tethers are controlled by control mechanisms.

The laws of change of thrust forces and tension forces are defined, ensuring that the system is rotated with a given angular velocity and given sufficiently large finite length of tethers.

In the final state, the satellite system is a square with significantly larger size.

The problem is reduced to an initial problem for a system of differential equations, which is solved using the **Runge-Kutta-Fehlberg 4(5) method**  *(RKF45)*.

### Mathematical model

$\text{1. General form of the differential equations system:}$

$$
\frac{dx}{dt} = F(x, t),
$$

$\text{where:}$

$\cdot \text{ } x = \lparen V, l, \omega_{\theta}, \theta \rparen^{T} - \text{the system state vector,}$  
$\cdot \text{ } l \text{ and } \theta - \text{the tethers length and the angle defining the square position,}$  
$\cdot \text{ } V = \frac{dl}{dt},$  
$\cdot \text{ } \omega_{\theta} = \frac{d \theta}{dt}.$

$\text{2. Motion equations:}$

$$
\frac{dV}{dt} = l \cdot \omega_{\theta}^{2} - \sqrt{2} \cdot \frac{F}{m} \cdot \cos{\lparen \varphi + \frac{\pi}{4} \rparen} - \frac{2 \cdot T}{m},
$$

$$
\frac{dl}{dt} = V,
$$

$$
\frac{d \omega_{\theta}}{dt} = \sqrt{2} \cdot \frac{F}{l \cdot m} \cdot \sin{\lparen \varphi + \frac{\pi}{4} \rparen} - \frac{2 \cdot V}{l} \cdot \omega_{\theta},
$$

$$
\frac{d \theta}{dt} = \omega_{\theta},
$$

$\text{where:}$

$\cdot \text{ } F - \text{thrust forces magnitude,}$  
$\cdot \text{ } T - \text{tethers tension forces magnitude,}$  
$\cdot \text{ } m - \text{microsatellites mass,}$  
$\cdot \text{ } \varphi - \text{thrust forces direction angle.}$

$\text{3. Thrust forces change program:}$

$$
F(t) = \begin{cases}
F_{0}, \text{ } t \le t_{1} \\
0, \text{ } t_{1} < t \le t_{2} \\
F_{0}, \text{ } t_{2} < t \le t_{3} \\
0, \text{ } t > t_{3}
\end{cases}
$$

$\text{where } t_{1,2,3} - \text{specified points in time.}$

$\text{4. Tethers tension forces change law:}$

$$
T = \frac{1}{2} \lbrack m \cdot l \cdot \omega_{\theta}^{2} + k_{l} \cdot \lparen l - l_{k} \rparen + k_{v} \cdot V \rbrack,
$$

$\text{where:}$

$\cdot \text{ } k_{l} - \text{length regulation ratio,}$  
$\cdot \text{ } k_{v} - \text{velocity regulation ratio,}$  
$\cdot \text{ } l_{k} - \text{target tethers length.}$
