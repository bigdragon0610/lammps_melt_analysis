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

# 線のみの等高線を描画
contour = plt.contour(xi, yi, zi, 30, colors='black')
plt.clabel(contour, inline=1, fontsize=10)  # 等高線にラベルを追加

plt.xlabel('X-axis')
plt.ylabel('Y-axis')
plt.title('Contour Plot')

plt.show()
