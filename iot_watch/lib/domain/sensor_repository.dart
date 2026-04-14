// lib/domain/sensor_repository.dart
import 'dart:async';
// Import the generated FRB class as our single source of truth
import 'package:iot_watch/src/rust/api.dart'; 

abstract class SensorRepository {
  // A broadcast stream containing the full state of the watch
  Stream<WatchUiState> get watchStateStream;
}