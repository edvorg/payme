[![pipeline status](https://gitlab.com/edvorg/payme/badges/master/pipeline.svg)](https://gitlab.com/edvorg/payme/commits/master)
[![coverage report](https://gitlab.com/edvorg/payme/badges/master/coverage.svg)](https://gitlab.com/edvorg/payme/commits/master)

# Payme

Invoice your client in one click.

## Overview

Tired of your invoicing flow? Me too.
This service generates invoices for your client in one click.
Don\'t beleive me? [Count it!](https://payme.rust.cafe)

## Usage

The usage is very simple. My invoicing flow is as follows:

1. At the end of the month open [payme](https://payme.rust.cafe) and send an invoice to your client

![Invoicing](img/01.png)

2. You'll receive an invoice copy by email with confirm link.

![Confirmation](img/02.png)

3. Pin invoice email.

![Pin confirmation](img/03.png)

4. Once payment is received, open confirmation email, click on confirm link and your client will receive a receipt

![Pin confirmation](img/04.png)

5. Archive receipt and invoice emails

![Pin confirmation](img/06.png)
![Pin confirmation](img/07.png)

6. Repeat. Payme will store your data in cookies for your convenince and in db until you clicked confirm link which removes all data related to your invoice
