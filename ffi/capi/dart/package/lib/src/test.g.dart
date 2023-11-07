import 'lib.g.dart';

class CodePointRangeIterator {
  final CodePointRangeIteratorImpl _impl;

  CodePointRangeIterator._(this._impl);

  CodePointRangeIteratorResult next() => _impl.next();
}
