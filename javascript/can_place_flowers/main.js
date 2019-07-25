/**
 * @param {number[]} flowerbed
 * @param {number} n
 * @return {boolean}
 */
var canPlaceFlowers = function (flowerbed, n) {
    if (0 == n) {
        return true;
    }
    for (let i = 0; i < flowerbed.length; i++) {
        if (0 == flowerbed[i]
            && (0 == i || 0 == flowerbed[i - 1])
            && (flowerbed.length - 1 == i || 0 == flowerbed[i + 1])) {
            flowerbed[i] = 1;
            n -= 1;
            if (0 == n) {
                return true;
            }
        }
    }
    return false;
};