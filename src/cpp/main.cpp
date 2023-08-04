#include <pybind11/pybind11.h>

#include "numericalField.h"
#include "stringField.h"

using namespace pybind11::literals;

namespace py = pybind11;

PYBIND11_MODULE(frcLogLens_cpp, m) {
  m.doc() = "CPP Binding Module for frcLogLens";

  py::class_<numericalField>(m, "numericalField")
      .def(py::init<std::map<double, double> &>())
      .def("scalarMultiply", &numericalField::scalarMultiply,
           "Returns a new field of each current entry multiplied by the given "
           "scalar.",
           "scalar"_a)
      .def("scalarDivide", &numericalField::scalarDivide,
           "Divides each entry by the given scalar. 0/0 is defined to be 0.",
           "scalar"_a)
      .def("scalarAdd", &numericalField::scalarAdd,
           "Adds the given scalar to each entry.", "scalar"_a)
      .def("multiply", &numericalField::multiply,
           "Returns a new field for every timestamp where there is a new value "
           "of either field, filled with the product of the two values at that "
           "time.",
           "other"_a)
      .def("divide", &numericalField::divide,
           "Returns a new field for every timestamp where there is a new value "
           "of either field, filled with the quotient of the two values at "
           "that time. 0/0 is defined to be 0.",
           "other"_a)
      .def("add", &numericalField::add,
           "Returns a new field for every timestamp where there is a new value "
           "of either field, filled with the sum of the two values at that "
           "time.",
           "other"_a)
      .def("subtract", &numericalField::subtract,
           "Returns a new field for every timestamp where there is a new value "
           "of either field, filled with the difference of the two values at "
           "that time.",
           "other"_a)
      .def("finiteDifference", &numericalField::finiteDifference,
           "Returns a new field of the finite time-difference of the current "
           "entries. 0/0 is defined to be 0,")
      .def("finiteIntegral", &numericalField::finiteIntegral,
           "Returns a new field of the finite time-integral of the current "
           "entries.")
      .def("sum", &numericalField::sum,
           "Returns a new field of the sum of all preceding values.")
      .def("delta", &numericalField::deltaFromPrevious,
           "Returns a new field of the difference between the current entry "
           "and the previous entry. If there is no previous entry, this is "
           "defined as 0.");
}