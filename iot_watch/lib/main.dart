// lib/main.dart
import 'package:flutter/material.dart';
import 'package:iot_watch/src/rust/frb_generated.dart'; 
import 'package:iot_watch/domain/blackboard_adapter.dart';
import 'package:iot_watch/domain/event_driven_adapter.dart';
import 'package:iot_watch/app.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();

  // THE SWAP: Comment out one, uncomment the other. That is it.
  //final sensorRepository = BlackboardAdapter();
  final sensorRepository = EventDrivenAdapter();

  runApp(IoTWatchApp(repository: sensorRepository));
}