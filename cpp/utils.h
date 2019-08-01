#include <iostream>
#include <vector>

template <typename T>
std::ostream &operator<<(std::ostream &os, const std::vector<T> &vec) {
  if (vec.empty()) {
    os << "[]";
    return os;
  }
  int len = vec.size();
  if (1 == len) {
    os << "[ " << vec[0] << " ]";
    return os;
  }
  os << "[ ";
  for (int i = 0; i < len - 1; ++i) {
    os << vec[i] << ", ";
  }
  os << vec[len - 1] << " ]";

  return os;
}