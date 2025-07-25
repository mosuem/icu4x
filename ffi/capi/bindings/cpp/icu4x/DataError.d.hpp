#ifndef icu4x_DataError_D_HPP
#define icu4x_DataError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum DataError {
      DataError_Unknown = 0,
      DataError_MarkerNotFound = 1,
      DataError_IdentifierNotFound = 2,
      DataError_InvalidRequest = 3,
      DataError_InconsistentData = 4,
      DataError_Downcast = 5,
      DataError_Deserialize = 6,
      DataError_Custom = 7,
      DataError_Io = 8,
    };

    typedef struct DataError_option {union { DataError ok; }; bool is_ok; } DataError_option;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * Additional information: [1](https://docs.rs/icu_provider/2.0.0/icu_provider/struct.DataError.html), [2](https://docs.rs/icu_provider/2.0.0/icu_provider/enum.DataErrorKind.html)
 */
class DataError {
public:
  enum Value {
    Unknown = 0,
    MarkerNotFound = 1,
    IdentifierNotFound = 2,
    InvalidRequest = 3,
    InconsistentData = 4,
    Downcast = 5,
    Deserialize = 6,
    Custom = 7,
    Io = 8,
  };

  DataError(): value(Value::Unknown) {}

  // Implicit conversions between enum and ::Value
  constexpr DataError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline icu4x::capi::DataError AsFFI() const;
  inline static icu4x::DataError FromFFI(icu4x::capi::DataError c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_DataError_D_HPP
