// lib/domain/event_driven_adapter.dart
import 'dart:async';
import 'package:iot_watch/domain/sensor_repository.dart';
import 'package:iot_watch/src/rust/api.dart'; // The generated FRB file

class EventDrivenAdapter implements SensorRepository {
  final _stateController = StreamController<WatchUiState>.broadcast();

  EventDrivenAdapter() {
    // Calls the Event-Driven Rust function!
    startEventDrivenSimulation().listen((state) {
      _stateController.add(state);
    });
  }

  @override
  Stream<WatchUiState> get watchStateStream => _stateController.stream;
}