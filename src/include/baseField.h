#pragma once

#include <algorithm>
#include <vector>

template <class F> struct entry {
  F value;
  double timestamp;

  entry(F value, double timestamp) : value(value), timestamp(timestamp) {}
};

template <class T> class baseField {
public:
  baseField(std::vector<T, class Allocator<T>> values,
            std::vector<double, std::allocator<double>> timestamps)
      : values(values), timestamps(timestamps) {
    stableSort();
  }

  baseField(std::vector<entry<T>> entries) {
    for (entry<T> entry : entries) {
      values.push_back(entry.value);
      timestamps.push_back(entry.timestamp);
    }
    stableSort();
  }

  std::vector<T> getValuesAtTimestamp(double timestamp);

  T getValueAtIndex(int index) { return values.at(index); }

  std::vector<int> getIndexesOfValue(T value);

  std::vector<double> getTimestampsOfValue(T value);

  std::vector<entry<T>> getEntrySet() {
    std::vector<entry<T>> entrySet{std::min(values.size(), timestamps.size())};

    for (size_t i = 0; i < std::min(values.size(), timestamps.size()); i++) {
      entrySet.push_back(entry<T>{values.at(i), timestamps.at(i)});
    }

    return entrySet;
  }

  std::vector<T, Allocator<T>> getValueSet() { return values; }

  std::vector<double, std::allocator<double>> getTimestampSet() {
    return timestamps;
  }

  baseField<T> squash();

  baseField<T> appendMerge(std::vector<baseField<T>> others);

  baseField<T> mergeInPlace(std::vector<baseField<T>> others);

private:
  std::vector<T, Allocator<T>> values{};
  std::vector<double, std::allocator<double>> timestamps{};

  void stableSort();
};