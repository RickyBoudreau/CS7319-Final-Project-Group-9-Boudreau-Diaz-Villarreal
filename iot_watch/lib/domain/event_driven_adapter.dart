import 'dart:async';
import 'package:iot_watch/domain/sensor_repository.dart';
import 'package:iot_watch/src/rust/api.dart'; 

class EventDrivenAdapter implements SensorRepository {
  final _stateController = StreamController<WatchUiState>.broadcast();

  EventDrivenAdapter() {
    startEventDrivenSimulation().listen((state) {
      _stateController.add(state);
    });
  }

  @override
  Stream<WatchUiState> get watchStateStream => _stateController.stream;

  @override
  void openApp(String appId) {
    notifyAppOpened(appId: appId); // Calls Rust
  }

  @override
  void closeApp(String appId) {
    notifyAppClosed(appId: appId); // Calls Rust
  }
}