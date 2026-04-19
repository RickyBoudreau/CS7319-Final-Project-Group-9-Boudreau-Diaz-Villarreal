import 'dart:async';
import 'package:iot_watch/domain/sensor_repository.dart';
import 'package:iot_watch/src/rust/api.dart';

class BlackboardAdapter implements SensorRepository {
  final _stateController = StreamController<WatchUiState>.broadcast();

  BlackboardAdapter() {
    startBlackboardSimulation().listen((state) {
      _stateController.add(state);
    });
  }

  @override
  Stream<WatchUiState> get watchStateStream => _stateController.stream;

  @override
  void openApp(String appId) {
    // Intentional No-op for Blackboard
  }

  @override
  void closeApp(String appId) {
    // Intentional No-op for Blackboard
  }
}