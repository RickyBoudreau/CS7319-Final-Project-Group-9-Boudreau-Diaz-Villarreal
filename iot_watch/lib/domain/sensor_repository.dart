import 'dart:async';
import 'package:iot_watch/src/rust/api.dart';

abstract class SensorRepository {
  Stream<WatchUiState> get watchStateStream;

  void openApp(String appId);
  void closeApp(String appId);
}