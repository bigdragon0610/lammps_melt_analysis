import numpy as np
import matplotlib.pyplot as plt
from scipy.interpolate import griddata

# ファイルからデータを読み取る
data = np.loadtxt("./dat/grid.dat", delimiter=" ")

x = data[:, 0]
y = data[:, 1]
z = data[:, 2]

# データをグリッドにマッピング
xi, yi = np.linspace(x.min(), x.max(), 100), np.linspace(y.min(), y.max(), 100)
xi, yi = np.meshgrid(xi, yi)
zi = griddata((x, y), z, (xi, yi), method='cubic')

# 等高線を描画
plt.contourf(xi, yi, zi, 15, cmap=plt.cm.jet)
plt.colorbar()
plt.scatter(x, y, c=z, s=50, cmap=plt.cm.jet)
plt.xlabel('r')
plt.ylabel('z_0')
# plt.title('Contour Plot')

plt.show()
