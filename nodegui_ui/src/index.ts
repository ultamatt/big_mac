import { QMainWindow, QWidget, QLabel, FlexLayout } from '@nodegui/nodegui';
const bm = require('big_mac');

const win = new QMainWindow();
win.setWindowTitle("Big Mac Display");

const centralWidget = new QWidget();
centralWidget.setObjectName("myroot");

const rootLayout = new FlexLayout();
centralWidget.setLayout(rootLayout);

const label = new QLabel();
label.setObjectName("mylabel");
label.setText("Your Mac is");

const label2 = new QLabel();
label.setObjectName("mymac");
label2.setText(bm.get_mac().toString());

rootLayout.addWidget(label);
rootLayout.addWidget(label2);
win.setCentralWidget(centralWidget);
win.setStyleSheet(
  `
    #myroot {
      background-color: #1E1E24;
      height: '100%';
      align-items: 'center';
      justify-content: 'center';
    }
    #mylabel {
      color: #81AE9D;
      font-size: 24px;
      font-weight: bold;
      padding: 1;
    }
    #mymac {
      color: #21A179;
      font-size: 20px;
      font-weight: bold;
      padding: 1;
    }
  `
);
win.show();

(global as any).win = win;
