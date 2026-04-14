// lib/domain/blackboard_adapter.dart
import 'dart:async';
import 'package:iot_watch/domain/sensor_repository.dart';
import 'package:iot_watch/src/rust/api.dart';

class BlackboardAdapter implements SensorRepository {
  final _stateController = StreamController<WatchUiState>.broadcast();

  BlackboardAdapter() {
    _initSimulation();
  }

  void _initSimulation() {
    // We just pipe the FRB generated state directly into our controller!
    startBlackboardSimulation().listen((state) {
      _stateController.add(state);
    });
  }

  @override
  Stream<WatchUiState> get watchStateStream => _stateController.stream;

  void dispose() {
    _stateController.close();
  }
}